//! Default plugin example

extern crate bevy_rosc;

use bevy::prelude::*;

use bevy_rosc::OscMethod;
use bevy_rosc::{BevyRoscPlugin, SingleAddressOscMethod};

/// Startup system that just spawns some entity bundles
fn startup(mut commands: Commands) {
    println!("** Startup");

    // Spawn entity with OSC receiving component that has a single address
    commands
        .spawn()
        .insert(SingleAddressOscMethod::new("/test/address".into()).unwrap());
}

/// System that listens for any `SingleAddressOscMethod` that has changed and then prints out the oldest received OscMessage
fn print_received_osc_packets(
    mut query: Query<&mut SingleAddressOscMethod, Changed<SingleAddressOscMethod>>,
) {
    for mut osc_receiver in query.iter_mut() {
        let new_msg = osc_receiver.get_message();
        if let Some(msg) = new_msg {
            println!(
                "Method {:?} received: {:?}",
                osc_receiver.get_addresses(),
                msg
            )
        }
    }
}

fn main() {
    App::new()
        // Minimal Bevy plugins
        .add_plugins(MinimalPlugins)
        // Add the bevy_rosc plugin and have it listen on port 31337
        .add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
        .add_startup_system(startup)
        .add_system(print_received_osc_packets)
        .run();
}
