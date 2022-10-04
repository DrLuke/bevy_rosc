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
    pub fn dispatch(&mut self, osc_packets: Vec<OscPacket>, event_writer: EventWriter<OscDispatchEvent>) {
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
        self.dispatch_messages(osc_messages, event_writer);
    }

    /// Dispatch multiple messages at once (i.e. from a bundle)
    fn dispatch_messages(&mut self, osc_messages: Vec<OscMessage>, mut event_writer: EventWriter<OscDispatchEvent>) -> Result<(), OscError> {
        let mut messages = vec![];

        for osc_message in &osc_messages {
            let matcher = match self.matchers.entry(String::from(osc_message.addr.as_str())) {
                // Create matchers for address patterns if they don't yet exist
                Entry::Vacant(o) => o.insert(Matcher::new(osc_message.addr.as_str())?).clone(),
                Entry::Occupied(o) => o.get().clone()
            };
            messages.push((matcher, osc_message.clone()))
        }

        event_writer.send(OscDispatchEvent { messages });

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

/// An event containing all OSC messages that were received this frame and their corresponding
/// [`rosc::address:Matcher`]s
pub struct OscDispatchEvent {
    pub messages: Vec<(Matcher, OscMessage)>,
}

/// This reads [`OscDispatcherEvent`]s sent by the dispatcher and forwards the incoming messages
/// to [`OscMethod`]s
pub fn method_dispatcher_system<T: OscMethod + Component>(
    mut event_reader: EventReader<OscDispatchEvent>,
    mut osc_method_query: Query<&mut T>,
) {
    for ev in event_reader.iter() {
        for mut osc_method in osc_method_query.iter_mut() {
            for (matcher, message) in &ev.messages {
                osc_method.match_message(matcher, message);
            }
        }
    }
}
