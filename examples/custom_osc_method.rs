//! Add a custom component implementing OscMethod and have it receive messages

extern crate bevy_rosc;

use bevy::prelude::*;
use rosc::address::OscAddress;
use rosc::OscMessage;

use bevy_rosc::{BevyRoscPlugin, method_dispatcher_system};
use bevy_rosc::OscMethod;

#[derive(Component)]
struct MyOscMethod {
    osc_address: OscAddress,
}

impl OscMethod for MyOscMethod {
    fn get_addresses(&self) -> Vec<OscAddress> {
        return vec![self.osc_address.clone()];
    }

    // This method is called when an OSC message was successfully matched with the method
    fn receive_message(&mut self, osc_message: OscMessage) {
        println!("MyOscMethod received: {:?}", osc_message)
    }
}

fn startup(mut commands: Commands) {
    println!("** Startup");

    // Spawn entity with custom OSC receiving component
    commands.spawn(MyOscMethod{osc_address: OscAddress::new("/test/address".into()).unwrap()});
}

fn main() {
    App::new()
        // Minimal Bevy plugins
        .add_plugins(MinimalPlugins)
        // Add the bevy_rosc plugin and have it listen on port 31337
        .add_plugins(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
        // Add dispatcher system for MyOscMethod
        .add_systems(Update, method_dispatcher_system::<MyOscMethod>)
        .add_systems(Startup, startup)
        .run();
}
