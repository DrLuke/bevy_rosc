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
    event_received.0 = event_reader.read().next().is_some();
}

#[test]
/// Minimal test of dispatcher. Just generates a single OSC message and see if it generates an
/// `OscDispatchEvent`.
fn dispatch_osc_message() {
    let mut app = App::new();
    app.add_event::<OscDispatchEvent>();
    app.insert_resource(Wrapper(false));
    app.add_systems(Update, dispatch_single);
    app.add_systems(Update, check_event_single.after(dispatch_single));

    app.insert_resource(OscDispatcher::default());

    app.update();

    // Resource is set to true if event was received
    assert!(app.world().resource::<Wrapper<bool>>().0)
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
    received_msgs.0 = match event_reader.read().next() {
        Some(e) => e.messages.clone(),
        None => vec![],
    };
}

#[test]
/// Same as above, but with an `OscBundle`, which has to be unpacked into it's constituent messages
/// before writing the event. We have to check that all messages found their way into the event.
fn dispatch_osc_bundle() {
    let mut app = App::new();
    app.add_event::<OscDispatchEvent>();

    // Messages that were included in the event
    let msgs: Wrapper<Vec<(Matcher, OscMessage)>> = Wrapper(vec![]);
    app.insert_resource(msgs);

    app.add_systems(Update, (
        dispatch_bundle,
        check_event_bundle.after(dispatch_bundle)
    ));

    app.insert_resource(OscDispatcher::default());

    app.update();

    // Resource is set to true if event was received
    let received_msgs = app.world_mut().resource::<Wrapper<Vec<(Matcher, OscMessage)>>>();
    assert_eq!(3, received_msgs.0.len());
    assert_eq!("/entity1/value", received_msgs[0].1.addr);
    assert_eq!("/entity2/value", received_msgs[1].1.addr);
    assert_eq!("/entity3/value", received_msgs[2].1.addr);
}
