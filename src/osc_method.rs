use bevy::prelude::*;
use rosc::{OscError, OscMessage};
use rosc::address::{Matcher, OscAddress};
use std::collections::VecDeque;

/// An OSC Method is capable of receiving OSC messages at one or multiple addresses.
pub trait OscMethod {
    /// Returns all the addresses of this OSC method
    fn get_addresses(&self) -> Vec<OscAddress>;
    /// Receive an OSC message and do something with it, like storing it in a receive queue
    fn receive_message(&mut self, osc_message: OscMessage);
    /// Check if an OSC message's address pattern matches with the method's address and receive it
    /// if it does. Also returns true if it's a match, and false otherwise.
    ///
    /// # Arguments
    ///
    /// * `matcher` The precomputed `rosc::address::Matcher` of the `rosc::types::OscMessage`'s address pattern
    ///
    /// * `message` The `rosc::types::OscMessage` that is being checked
    fn match_message(&mut self, matcher: &Matcher, message: &OscMessage) -> bool {
        for addr in &self.get_addresses() {
            if matcher.match_address(addr) {
                self.receive_message(message.clone());
                return true;
            }
        }
        false
    }
}

/// Bevy component that can receive OSC messages at one or multiple addresses
#[derive(Component)]
pub struct MultiAddressOscMethod {
    /// Valid OSC addresses
    addresses: Vec<OscAddress>,
    /// Received OSC messages that matched one of the addresses
    messages: VecDeque<OscMessage>,
}

impl MultiAddressOscMethod {
    /// Gets the oldest message from the message queue
    pub fn get_message(&mut self) -> Option<OscMessage> { self.messages.pop_front() }

    /// Returns a new `OscMethod`
    ///
    /// # Arguments
    ///
    /// * `addresses` - A valid OSC address. Must start with a `/` and must only contain printable ASCII characters except for ` `(space), `#`, `*`, `,`, `?`, `[`, `]`, `{`, `}`. For example, `/foo/bar/123` would be a valid address.
    ///
    /// # Errors
    ///
    /// This function will return a [BadAddress](rosc::OscError::BadAddress) error when the address is invalid.
    pub fn new(addresses: Vec<String>) -> Result<Self, OscError> {
        let osc_addresses: Result<Vec<OscAddress>, _> = addresses.into_iter().map(|a| OscAddress::new(a)).collect();

        Ok(Self {
            addresses: osc_addresses?,
            messages: Default::default(),
        })
    }
}

impl OscMethod for MultiAddressOscMethod {
    fn get_addresses(&self) -> Vec<OscAddress> { self.addresses.clone() }
    fn receive_message(&mut self, osc_message: OscMessage) { self.messages.push_back(osc_message) }
}
