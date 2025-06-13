use crate::osc_dispatcher::{method_dispatcher_system, OscDispatchEvent};
use crate::osc_method::SingleAddressOscMethod;
use crate::{MultiAddressOscMethod, OscDispatcher, OscUdpServer};
use bevy::prelude::*;
use std::io;
use std::net::ToSocketAddrs;

/// Plugin implementing the default functionality for bevy_rosc
///
/// It opens a single UDP server, and adds dispatching systems for both single and multi address
/// osc methods.
pub struct BevyRoscPlugin<A: ToSocketAddrs + Sync + 'static + Clone> {
    addrs: A,
}

impl<A: ToSocketAddrs + Send + Sync + 'static + Clone> BevyRoscPlugin<A> {
    pub fn new(addrs: A) -> Result<Self, io::Error> {
        Ok(BevyRoscPlugin { addrs })
    }
}

fn osc_receive_system(
    mut osc_dispatcher: ResMut<OscDispatcher>,
    mut query: Query<&mut OscUdpServer>,
    event_writer: EventWriter<OscDispatchEvent>,
) {
    let mut osc_packets = vec![];
    for osc_udp_server in query.iter_mut() {
        loop {
            if let Ok(o) = osc_udp_server.recv() {
                match o {
                    Some(p) => osc_packets.push(p),
                    None => break,
                }
            }
        }
    }

    osc_dispatcher.dispatch(osc_packets, event_writer);
}

impl<A: ToSocketAddrs + Send + Sync + 'static + Clone> Plugin for BevyRoscPlugin<A> {
    fn build(&self, app: &mut App) {
        app.insert_resource(OscDispatcher::default())
            .add_event::<OscDispatchEvent>()
            .add_systems(
                PreUpdate,
                (
                    osc_receive_system,
                    method_dispatcher_system::<SingleAddressOscMethod>.after(osc_receive_system),
                    method_dispatcher_system::<MultiAddressOscMethod>.after(osc_receive_system),
                ),
            );
        app.world_mut()
            .spawn(OscUdpServer::new(self.addrs.clone()).unwrap());
    }
}
