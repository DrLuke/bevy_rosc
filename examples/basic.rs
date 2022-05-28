//! A minimal example for how to work with the dispatcher.
//!
//! The dispatcher is responsible for
//!

extern crate bevy_osc;

use bevy::core::FixedTimestep;
use bevy::prelude::*;

use bevy_osc::OscMethod;
use bevy_osc::OscDispatcher;
use rosc::OscMessage;
use rosc::OscPacket;

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

    for i in 0..3 {
        commands.spawn_bundle(ExampleBundle {
            _t: ExampleEntity,
            receiver: OscMethod::new(vec![format!("/entity{}/time", i).as_str()]).expect(""),
        });
    }
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

/// Creates an `OscMessage` and then dispatches it
fn send_message(mut disp: ResMut<OscDispatcher>, time: Res<Time>, method_query: Query<&mut OscMethod>) {
    let new_msg = OscMessage { addr: "/entity*/time".to_string(), args: vec![time.time_since_startup().as_secs_f32().into()] };

    println!("Dispatching: {:?}", new_msg);

    disp.dispatch(vec![OscPacket::Message(new_msg)], method_query);
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)

        // Add dispatcher resource
        .insert_resource(OscDispatcher::default())

        .add_startup_system(startup)
        .add_system(print_received_osc_packets)

        // Send one OSC Message per second
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(send_message)
        )

        .run();
}
