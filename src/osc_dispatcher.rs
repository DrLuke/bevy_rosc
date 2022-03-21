use std::collections::hash_map::Entry;
use std::collections::HashMap;
use bevy::prelude::*;
use rosc::{OscBundle, OscError, OscMessage, OscPacket};
use rosc::address::Matcher;
use crate::OscMethod;

/// Dispatches received [OscPacket](rosc::OscPacket)s to all [OscMethod]s with a matching address
#[derive(Default)]
#[derive(Component)]
pub struct OscDispatcher {
    matchers: HashMap<String, Matcher>,
}

impl OscDispatcher {
    /// Dispatch `OscPacket`s to all matching `OscMethod`s
    pub fn dispatch(&mut self, osc_packets: Vec<OscPacket>, query: Query<&mut OscMethod>) {
        let osc_messages = osc_packets
            .into_iter()
            .map(
                |osc_packet| {
                    match osc_packet {
                        OscPacket::Message(message) => vec![message],
                        OscPacket::Bundle(bundle) => OscDispatcher::unpack_bundle(bundle)
                    }
                }
            )
            .flatten()
            .collect();
        self.dispatch_multiple_messages(osc_messages, query);
    }

    /// Dispatch multiple messages at once (i.e. from a bundle)
    fn dispatch_multiple_messages(&mut self, osc_messages: Vec<OscMessage>, mut query: Query<&mut OscMethod>) -> Result<(), OscError> {
        let mut matchers = vec![];

        // Create matchers for address patterns if they don't yet exist
        for osc_message in &osc_messages {
            if let Entry::Vacant(o) = self.matchers.entry(String::from(osc_message.addr.as_str())) {
                o.insert(Matcher::new(osc_message.addr.as_str())?);
            }
        }
        // Fetch matchers required for the osc messages
        for osc_message in &osc_messages {
            matchers.push(self.matchers.get(osc_message.addr.as_str()).expect(""));
        }

        for mut osc_receiver in query.iter_mut() {
            for (index, matcher) in matchers.iter().enumerate() {
                if matcher.match_address(osc_receiver.get_address()).expect("Address already validated") {
                    osc_receiver.receive_message(osc_messages[index].clone());
                }
            }
        }

        Ok(())
    }

    /// Recursively retrieve all `OscMessage`s from an `OscBundle`
    fn unpack_bundle(osc_bundle: OscBundle) -> Vec<OscMessage>
    {
        let mut messages = Vec::new();

        for osc_packet in osc_bundle.content {
            match osc_packet {
                OscPacket::Message(message) => messages.push(message),
                OscPacket::Bundle(bundle) => messages.extend(OscDispatcher::unpack_bundle(bundle))
            }
        }

        messages
    }
}

