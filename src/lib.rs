//! This is a plugin implementing OSC for bevy using `rosc`.
//!
//!
//!
//! # Basic usage
//!
//! Add the [`BevyRoscPlugin`](plugin::BevyRoscPlugin) to your app, choosing the ip address at which you want to receive at.
//! Then add either the [`SingleAddressOscMethod`](osc_method::SingleAddressOscMethod) or [`MultiAddressOscMethod`](osc_method::MultiAddressOscMethod) component to your entity.
//! `bevy_rosc` will automatically deliver matching messages to the component, where you can then retrieve them with the [`get_address`](osc_method::MultiAddressOscMethod::get_message) method.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_rosc::OscDispatcher;
//! use bevy_rosc::SingleAddressOscMethod;
//! use bevy_rosc::OscMethod;
//! use bevy_rosc::BevyRoscPlugin;
//!
//! fn spawn(mut commands: Commands) {
//!     commands
//!         .spawn()
//!         .insert(SingleAddressOscMethod::new("/test/address".into()).unwrap());
//! }
//!
//! fn print_received_osc_packets(mut query: Query<&mut SingleAddressOscMethod, (Changed<SingleAddressOscMethod>)>) {
//!     for mut osc_method in query.iter_mut() {
//!         match osc_method.get_message() {
//!             Some(message) => println!("Method {:?} received: {:?}", osc_method.get_address(), message),
//!             None => {}
//!         }
//!     }
//! }
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(MinimalPlugins)
//!         .add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
//!         .add_startup_system(startup)
//!         .add_system(print_received_osc_packets)
//!         .run();
//! }
//! ```
extern crate rosc;

mod osc_dispatcher;
mod osc_method;
mod osc_udp_client;
mod osc_udp_server;
mod plugin;

pub use osc_dispatcher::{method_dispatcher_system, OscDispatchEvent, OscDispatcher};
pub use osc_method::{MultiAddressOscMethod, OscMethod, SingleAddressOscMethod};
pub use osc_udp_client::OscUdpClient;
pub use osc_udp_server::OscUdpServer;
pub use plugin::BevyRoscPlugin;
