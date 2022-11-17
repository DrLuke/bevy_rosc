//! A minimal example setting up bevy_rosc without the plugin

extern crate bevy_rosc;

use bevy::prelude::*;

use bevy_rosc::OscMethod;
use bevy_rosc::{method_dispatcher_system, MultiAddressOscMethod, OscDispatchEvent};
use bevy_rosc::{OscDispatcher, OscUdpServer};

fn startup(mut commands: Commands) {
    println!("** Startup");

    commands.spawn(MultiAddressOscMethod::new(vec!["/test/address".into()]).unwrap());

    // Spawn UDP server that can receive OSC packets on port 31337
    commands.spawn(OscUdpServer::new("0.0.0.0:31337").unwrap());
}

/// System that listens for any `MultiAddressOscMethod` that has changed and then prints out the received OscMessage
fn print_received_osc_packets(
    mut query: Query<&mut MultiAddressOscMethod, Changed<MultiAddressOscMethod>>,
) {
    for mut osc_method in query.iter_mut() {
        let new_msg = osc_method.get_message();
        if let Some(msg) = new_msg {
            println!(
                "Method {:?} received: {:?}",
                osc_method.get_addresses()[0],
                msg
            )
        }
    }
}

/// System that receives messages via UDP and then forwards them to the dispatcher
fn osc_receive_system(
    mut osc_dispatcher: ResMut<OscDispatcher>,
    mut query: Query<&mut OscUdpServer>,
    event_writer: EventWriter<OscDispatchEvent>,
) {
    let mut osc_packets = vec![];
    for osc_udp_server in query.iter_mut() {
        loop {
            if let Ok(o) = osc_udp_server.recv() {
                match o {
                    Some(p) => osc_packets.push(p),
                    None => break,
                }
            }
        }
    }

    osc_dispatcher.dispatch(osc_packets, event_writer);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        // Add dispatcher resource
        .insert_resource(OscDispatcher::default())
        // Event sent by the dispatcher
        .add_event::<OscDispatchEvent>()
        // System that received the dispatch event and attempts to match received messages with all `MultiAddressOscMethod` components
        .add_system(method_dispatcher_system::<MultiAddressOscMethod>)
        .add_startup_system(startup)
        .add_system(print_received_osc_packets)
        .add_system(osc_receive_system)
        .run();
}
