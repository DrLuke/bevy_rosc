//! UDP server example

extern crate bevy_osc;

use bevy::prelude::*;

use bevy_osc::{OscMethod, OscMultiMethod};
use bevy_osc::OscDispatcher;
use bevy_osc::OscUdpServer;

#[derive(Component)]
struct ExampleEntity;

#[derive(Bundle)]
#[derive(Component)]
struct ExampleBundle {
    _t: ExampleEntity,
    receiver: OscMethod,
}

/// Startup system that just spawns some entity bundles
fn startup(mut commands: Commands) {
    println!("** Startup");

    // Spawn a bundle with an OSC method that can have OSC packets dispatched to it
    commands.spawn_bundle(ExampleBundle {
        _t: ExampleEntity,
        receiver: OscMethod::new("/some/entity").expect(""),
    });

    // Spawn UDP server that can receive OSC packets on port 31337
    commands.spawn().insert(OscUdpServer::new("0.0.0.0:31337").unwrap());
}

/// System that listens for any `OscMethod` that has changed and then prints out the received OscMessage
fn print_received_osc_packets(mut query: Query<&mut OscMethod, Changed<OscMethod>>) {
    for mut osc_receiver in query.iter_mut() {
        let new_msg = osc_receiver.get_message();
        match new_msg {
            Some(msg) => {
                println!("Method {} received: {:?}", osc_receiver.get_address(), msg)
            }
            None => {}
        }
    }
}

/// Read `OscPacket`s from udp server until no more messages are received and then dispatches them
fn receive_packets(mut disp: ResMut<OscDispatcher>, mut query: Query<&mut OscUdpServer>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    let osc_udp_server = query.single_mut();
    let mut osc_packets = vec![];

    loop {
        match osc_udp_server.recv() {
            Ok(o) => match o {
                Some(p) => osc_packets.push(p),
                None => break
            },
            Err(_) => ()
        }
    }

    disp.dispatch(osc_packets, method_query, multi_method_query);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        // Add dispatcher resource
        .insert_resource(OscDispatcher::default())

        .add_startup_system(startup)
        .add_system(print_received_osc_packets)
        .add_system(receive_packets)

        .run();
}
