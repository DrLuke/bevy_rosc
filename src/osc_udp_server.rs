use std::io;
use std::io::ErrorKind;
use bevy::prelude::*;
use std::net::{ToSocketAddrs, UdpSocket};

use rosc::decoder::{decode_udp, MTU};
use rosc::{OscError, OscPacket};

use nom::Err;

#[derive(Component)]
pub struct OscUdpServer {
    socket: UdpSocket
}

#[derive(Debug)]
pub enum OscUdpReceiveError {
    OscError(OscError),
    IoError(io::Error)
}

impl OscUdpServer{
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(addr)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket
        })
    }

    pub fn recv(&self) -> Result<Option<OscPacket>, OscUdpReceiveError> {
        let mut buf = [0; MTU];

        let result = self.socket.recv(&mut buf);
        match result {
            Ok(num_bytes) => {
                match decode_udp(&buf[0..num_bytes]) {
                    Ok((_, osc_packet)) => Ok(Some(osc_packet)),
                    Err(e) => match e {
                        Err::Incomplete(_) => Err(OscUdpReceiveError::OscError(rosc::OscError::BadPacket("Incomplete data"))),
                        Err::Error(e) => Err(OscUdpReceiveError::OscError(e)),
                        Err::Failure(e) => Err(OscUdpReceiveError::OscError(e))
                    }
                }
            },

            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(err) => Err(OscUdpReceiveError::IoError(err))
        }
    }
}