//! UDP server example

extern crate bevy_rosc;

use bevy::prelude::*;

use bevy_rosc::OscMethod;
use bevy_rosc::OscDispatcher;
use bevy_rosc::OscUdpServer;

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
        receiver: OscMethod::new(vec!["/beat/mute"]).expect(""),
    });

    // Spawn UDP server that can receive OSC packets on port 31337
    commands.spawn().insert(OscUdpServer::new("0.0.0.0:31337").unwrap());
}

/// System that listens for any `OscMethod` that has changed and then prints out the received OscMessage
fn print_received_osc_packets(mut query: Query<&mut OscMethod, Changed<OscMethod>>) {
    for mut osc_receiver in query.iter_mut() {
        let new_msg = osc_receiver.get_message();
        if let Some(msg) = new_msg {
            println!("Method {} received: {:?}", osc_receiver.get_addresses()[0], msg)
        }
    }
}

/// Read `OscPacket`s from udp server until no more messages are received and then dispatch them
fn receive_packets(mut disp: ResMut<OscDispatcher>, mut query: Query<&mut OscUdpServer>, method_query: Query<&mut OscMethod>) {
    let osc_udp_server = query.single_mut();
    let mut osc_packets = vec![];

    loop {
        if let Ok(o) = osc_udp_server.recv() {
            match o {
                Some(p) => osc_packets.push(p),
                None => break
            }
        }
    }

    disp.dispatch(osc_packets, method_query);
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
