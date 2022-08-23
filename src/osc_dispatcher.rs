use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::DerefMut;
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
    pub fn dispatch(&mut self, osc_packets: Vec<OscPacket>, method_query: Query<&mut OscMethod>) {
        let osc_messages = osc_packets
            .into_iter()
            .flat_map(
                |osc_packet| {
                    match osc_packet {
                        OscPacket::Message(message) => vec![message],
                        OscPacket::Bundle(bundle) => OscDispatcher::unpack_bundle(bundle)
                    }
                }
            )
            .collect();
        self.dispatch_messages(osc_messages, method_query);
    }

    /// Dispatch multiple messages at once (i.e. from a bundle)
    fn dispatch_messages(&mut self, osc_messages: Vec<OscMessage>, mut method_query: Query<&mut OscMethod>) -> Result<(), OscError> {
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

        for mut osc_receiver in method_query.iter_mut() {
            for (index, matcher) in matchers.iter().enumerate() {
                osc_receiver.deref_mut().match_addresses(matcher, &osc_messages[index]);
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

