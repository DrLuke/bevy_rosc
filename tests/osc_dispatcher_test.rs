extern crate bevy_rosc;

use bevy::prelude::*;

use bevy_rosc::OscDispatchEvent;
use bevy_rosc::OscDispatcher;
use rosc::address::Matcher;
use rosc::OscPacket;
use rosc::{OscBundle, OscMessage, OscTime};

// Resource wrapper utility to use scalar types as resources
#[derive(Resource, Deref, DerefMut)]
pub struct Wrapper<T>(T);

fn dispatch_single(mut disp: ResMut<OscDispatcher>, event_writer: EventWriter<OscDispatchEvent>) {
    disp.dispatch(
        vec![OscPacket::Message(OscMessage {
            addr: "/foo".into(),
            args: vec![],
        })],
        event_writer,
    );
}

fn check_event_single(
    mut event_reader: EventReader<OscDispatchEvent>,
    mut event_received: ResMut<Wrapper<bool>>,
) {
    // Check if an event was received
    event_received.0 = event_reader.iter().next().is_some();
}

#[test]
/// Minimal test of dispatcher. Just generates a single OSC message and see if it generates an
/// `OscDispatchEvent`.
fn dispatch_osc_message() {
    let mut world = World::default();
    world.init_resource::<Events<OscDispatchEvent>>(); // Set up OscDispatchEvent to work

    // Bool used to check if event was received
    world.insert_resource(Wrapper(false));

    let mut update_stage = SystemStage::single_threaded();
    update_stage.add_system(dispatch_single.label("send"));
    update_stage.add_system(check_event_single.after("send"));

    world.insert_resource(OscDispatcher::default());

    update_stage.run(&mut world);

    // Resource is set to true if event was received
    assert!(world.resource::<Wrapper<bool>>().0)
}

fn dispatch_bundle(mut disp: ResMut<OscDispatcher>, event_writer: EventWriter<OscDispatchEvent>) {
    let new_msg = OscBundle {
        timetag: OscTime {
            seconds: 0,
            fractional: 0,
        },
        content: vec![
            OscPacket::Message(OscMessage {
                addr: "/entity1/value".to_string(),
                args: vec![],
            }),
            OscPacket::Bundle(OscBundle {
                timetag: OscTime {
                    seconds: 0,
                    fractional: 0,
                },
                content: vec![
                    OscPacket::Message(OscMessage {
                        addr: "/entity2/value".to_string(),
                        args: vec![],
                    }),
                    OscPacket::Message(OscMessage {
                        addr: "/entity3/value".to_string(),
                        args: vec![],
                    }),
                ],
            }),
        ],
    };

    disp.dispatch(vec![OscPacket::Bundle(new_msg)], event_writer);
}

fn check_event_bundle(
    mut event_reader: EventReader<OscDispatchEvent>,

    mut received_msgs: ResMut<Wrapper<Vec<(Matcher, OscMessage)>>>,
) {
    // Get all messages included in the event
    received_msgs.0 = match event_reader.iter().next() {
        Some(e) => e.messages.clone(),
        None => vec![],
    };
}

#[test]
/// Same as above, but with an `OscBundle`, which has to be unpacked into it's constituent messages
/// before writing the event. We have to check that all messages found their way into the event.
fn dispatch_osc_bundle() {
    let mut world = World::default();
    world.init_resource::<Events<OscDispatchEvent>>(); // Set up OscDispatchEvent to work

    // Messages that were included in the event
    let msgs: Wrapper<Vec<(Matcher, OscMessage)>> = Wrapper(vec![]);
    world.insert_resource(msgs);

    let mut update_stage = SystemStage::single_threaded();
    update_stage.add_system(dispatch_bundle.label("send"));
    update_stage.add_system(check_event_bundle.after("send"));

    world.insert_resource(OscDispatcher::default());

    update_stage.run(&mut world);

    // Resource is set to true if event was received
    let received_msgs = world.resource::<Wrapper<Vec<(Matcher, OscMessage)>>>();
    assert_eq!(3, received_msgs.0.len());
    assert_eq!("/entity1/value", received_msgs[0].1.addr);
    assert_eq!("/entity2/value", received_msgs[1].1.addr);
    assert_eq!("/entity3/value", received_msgs[2].1.addr);
}
