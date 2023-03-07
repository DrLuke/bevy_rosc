//! A minimal example setting up bevy_rosc without the plugin

extern crate bevy_rosc;

use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;

use bevy_rosc::OscDispatcher;
use bevy_rosc::OscMethod;
use bevy_rosc::{method_dispatcher_system, MultiAddressOscMethod, OscDispatchEvent};
use rosc::OscMessage;
use rosc::OscPacket;

fn startup(mut commands: Commands) {
    println!("** Startup");

    for i in 0..3 {
        commands.spawn(MultiAddressOscMethod::new(vec![format!("/entity{}/time", i)]).unwrap());
    }
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

/// Create an `OscMessage` and then dispatch it
/// This would usually be replaced with receiving messages from an UDP server or similar
fn send_message(
    mut disp: ResMut<OscDispatcher>,
    time: Res<Time>,
    event_reader: EventWriter<OscDispatchEvent>,
) {
    let new_msg = OscMessage {
        addr: "/entity*/time".to_string(),
        args: vec![time.elapsed_seconds().into()],
    };

    println!("Dispatching: {:?}", new_msg);

    disp.dispatch(vec![OscPacket::Message(new_msg)], event_reader);
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
        // Send one OSC Message per second
        .add_system(send_message.run_if(on_fixed_timer(Duration::from_secs(1))))
        .run();
}
