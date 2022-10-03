//! This is a plugin implementing OSC for bevy using `rosc`.
//!
//!
//!
//! # Usage
//!
//! First you need to add the [dispatcher](bevy_rosc::OscDispatcher) as a resource to your app.
//! Then add an [OscMethod](bevy_rosc::OscMethod) component to your entity.
//! The dispatcher will now deliver all OSC messages that match the method's address to your component.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_rosc::OscDispatcher;
//! use bevy_rosc::MultiAddressOscMethod;
//! use bevy_rosc::OscMethod;
//!
//! #[derive(Component)]
//! struct ExampleEntity;
//!
//! #[derive(Bundle)]
//! #[derive(Component)]
//! struct ExampleBundle {
//!     _t: ExampleEntity,
//!     receiver: MultiAddressOscMethod,
//! }
//!
//! fn spawn(mut commands: Commands) {
//!     commands.spawn_bundle(ExampleBundle {
//!             _t: ExampleEntity,
//!             receiver: MultiAddressOscMethod::new(vec!["/some/address".into()]).expect(""),
//!         });
//! }
//!
//! fn osc_printer(mut query: Query<&mut MultiAddressOscMethod, (Changed<MultiAddressOscMethod>)>) {
//!     for mut osc_method in query.iter_mut() {
//!         match osc_method.get_message() {
//!             Some(message) => println!("Method {:?} received: {:?}", osc_method.get_addresses()[0], message),
//!             None => {}
//!         }
//!     }
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(MinimalPlugins)
//!         .insert_resource(OscDispatcher::default())
//!         .add_system(osc_printer)
//!         .run()
//! }
//! ```
extern crate rosc;

mod osc_method;
mod osc_dispatcher;
mod osc_udp_server;
mod osc_udp_client;

pub use osc_method::{OscMethod, MultiAddressOscMethod};
pub use osc_dispatcher::OscDispatcher;
pub use osc_udp_server::OscUdpServer;
pub use osc_udp_client::OscUdpClient;