//! This is a plugin implementing OSC for bevy using [`rosc`].
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
//!
//! # Advanced usage
//! There is the option to add custom osc method components.
//! Your component just has to implement [`OscMethod`] and you need to add a [`method_dispatcher_system`] for it.
//! Now your component will receive OSC messages at it's address(es).
//! 
//! ```no_run
//! extern crate bevy_rosc;
//! use bevy::prelude::*;
//! use bevy_rosc::OscMethod;
//! use bevy_rosc::{method_dispatcher_system, BevyRoscPlugin};
//! use rosc::address::OscAddress;
//! use rosc::OscMessage;
//! 
//! #[derive(Component)]
//! struct MyOscMethod {
//!     osc_address: OscAddress,
//! }
//! 
//! impl OscMethod for MyOscMethod {
//!     fn get_addresses(&self) -> Vec<OscAddress> {
//!         return vec![self.osc_address.clone()];
//!     }
//! 
//!     // This method is called when an OSC message was successfully matched with the method
//!     fn receive_message(&mut self, osc_message: OscMessage) {
//!         println!("MyOscMethod received: {:?}", osc_message)
//!     }
//! }
//! 
//! fn startup(mut commands: Commands) {
//!     commands.spawn().insert(MyOscMethod {
//!         osc_address: OscAddress::new("/test/address".into()).unwrap(),
//!     });
//! }
//! 
//! fn main() {
//!     App::new()
//!         .add_plugins(MinimalPlugins)
//!         .add_plugin(BevyRoscPlugin::new("0.0.0.0:31337").unwrap())
//!         .add_system(method_dispatcher_system::<MyOscMethod>) // <-- Add dispatcher system for your method
//!         .add_startup_system(startup)
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
