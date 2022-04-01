use bevy::prelude::*;
use rosc::{OscError, OscMessage};
use rosc::address::verify_address;
use std::collections::VecDeque;

/// Bevy component that can receive OSC messages
#[derive(Component)]
pub struct OscMethod {
    /// A valid OSC address
    address: String,
    /// Received OSC messages that match the address
    messages: VecDeque<OscMessage>,
}

impl OscMethod {
    /// Gets the oldest message from the message queue
    pub fn get_message(&mut self) -> Option<OscMessage> { self.messages.pop_front() }
    pub fn get_address(&self) -> &str { self.address.as_str() }

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
    pub fn new(address: &str) -> Result<Self, OscError> {
        verify_address(address)?;

        Ok(Self {
            address: String::from(address),
            messages: Default::default(),
        })
    }
}

/// Bevy component containing multiple OSC methods
#[derive(Component)]
pub struct OscMultiMethod {
    pub methods: Vec<OscMethod>,
}