use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use bevy::prelude::*;
use rosc::{OscBundle, OscError, OscMessage, OscPacket};
use rosc::address::Matcher;
use crate::{OscMethod, OscMultiMethod};

/// Dispatches received [OscPacket](rosc::OscPacket)s to all [OscMethod]s with a matching address
#[derive(Default)]
#[derive(Component)]
pub struct OscDispatcher {
    matchers: HashMap<String, Matcher>,
}

impl OscDispatcher {
    /// Dispatch `OscPacket`s to all matching `OscMethod`s
    pub fn dispatch(&mut self, osc_packets: Vec<OscPacket>, mut method_query: Query<&mut OscMethod>, mut multi_method_query: Query<&mut OscMultiMethod>) {
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
        self.dispatch_multiple_messages(osc_messages, method_query.iter_mut().collect(), multi_method_query.iter_mut().collect());
    }

    /// Dispatch multiple messages at once (i.e. from a bundle)
    fn dispatch_multiple_messages(&mut self, osc_messages: Vec<OscMessage>, osc_methods: Vec<Mut<OscMethod>>, osc_multi_methods: Vec<Mut<OscMultiMethod>>) -> Result<(), OscError> {
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

        for mut osc_receiver in osc_methods {
            for (index, matcher) in matchers.iter().enumerate() {
                if matcher.match_address(osc_receiver.deref().get_address()).expect("Address already validated") {
                    osc_receiver.deref_mut().receive_message(osc_messages[index].clone());
                }
            }
        }

        for mut osc_multi_method in osc_multi_methods {
            for (index, matcher) in matchers.iter().enumerate() {
                // Check that at least one of the methods in the multimethod matches and only then dereference as mutable
                // This is done so that the change detection only triggers if there's an actual change
                if osc_multi_method.deref().methods.iter().any(|x| matcher.match_address(x.get_address()).expect("Address already validated") ) {
                    for osc_method in osc_multi_method.deref_mut().methods.iter_mut() {
                        osc_method.receive_message(osc_messages[index].clone())
                    }
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

