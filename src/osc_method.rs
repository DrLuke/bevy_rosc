use bevy::prelude::*;
use rosc::{OscError, OscMessage};
use rosc::address::{Matcher, verify_address};
use std::collections::VecDeque;

/// Bevy component that can receive OSC messages at one or multiple addresses
#[derive(Component)]
pub struct OscMethod {
    /// Valid OSC addresses
    addresses: Vec<String>,
    /// Received OSC messages that matched one of the addresses
    messages: VecDeque<OscMessage>,
}

impl OscMethod {
    /// Gets the oldest message from the message queue
    pub fn get_message(&mut self) -> Option<OscMessage> { self.messages.pop_front() }
    pub fn get_addresses(&self) -> Vec<String> { self.addresses.clone() }

    /// Receives an OSC message and stores it at the end of the message queue.
    /// This method is called by the OSC dispatcher after successfully matching an incoming OSC message's address pattern to the OSC method's address.
    pub fn receive_message(&mut self, osc_message: OscMessage) {
        self.messages.push_back(osc_message);
    }

    /// Returns a new `OscMethod`
    ///
    /// # Arguments
    ///
    /// * `address` - A valid OSC address. Must start with a `/` and must only contain printable ASCII characters except for ` `(space), `#`, `*`, `,`, `?`, `[`, `]`, `{`, `}`. For example, `/foo/bar/123` would be a valid address.
    ///
    /// # Errors
    ///
    /// This function will return a [BadAddress](rosc::OscError::BadAddress) error when the address is invalid.
    pub fn new(addresses: Vec<&str>) -> Result<Self, OscError> {
        for addr in &addresses {
            verify_address(addr)?;
        }

        Ok(Self {
            addresses: addresses.iter().map(|&a| String::from(a)).collect(),
            messages: Default::default(),
        })
    }

    pub fn match_addresses(&mut self, matcher: &Matcher, message: OscMessage) {
        for addr in &self.addresses {
            if matcher.match_address(addr.as_str()).expect("Address already validated") {
                self.receive_message(message);
                return;
            }
        }
    }
}
