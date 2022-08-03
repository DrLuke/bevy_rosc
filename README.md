# bevy_rosc

[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_rosc)](https://crates.io/crates/bevy_rosc)
[![docs.rs](https://docs.rs/bevy_rosc/badge.svg)](https://docs.rs/bevy_rosc)

Send and receive [OSC 1.0](https://github.com/CNMAT/OpenSoundControl.org/blob/master/spec-1_0.md) messages in bevy with [rosc](https://github.com/klingtnet/rosc).

## Usage

There are two core components to use OSC in bevy: The [`OscMethod`](src/osc_method.rs), a component that can receive OSC messages at one or more address, and the [`OscDispatcher`](src/osc_dispatcher.rs), which takes received OSC messages and delivers them to the matching OSC methods.

Start by adding an `OscMethod` to your entity
```rust
#[derive(Component)]
struct ExampleEntity;

#[derive(Bundle)]
#[derive(Component)]
struct ExampleBundle {
    _t: ExampleEntity,
    osc_method: OscMethod,
}

/// Spawn ExampleBundle
fn spawn_entity(mut commands: Commands) {
    commands.spawn_bundle(ExampleBundle {
        _t: ExampleEntity,
        osc_method: OscMethod::new(vec!["/some/osc/address"]).expect("Method address is valid"),
    });
}
```

Next you need to set up the dispatcher, which distributes received messages

```rust
fn send_message(mut disp: ResMut<OscDispatcher>, time: Res<Time>, method_query: Query<&mut OscMethod>) {
    // In this case we just create a message, but this is where you could add a UDP server for example
    let new_msg = OscMessage { addr: "/some/*/address".to_string(), args: vec![time.time_since_startup().as_secs_f32().into()] };

    disp.dispatch(vec![OscPacket::Message(new_msg)], method_query);
}
```

Then add a system that reacts to new messages!

```rust
fn print_received_osc_packets(mut query: Query<(&ExampleEntity, &mut OscMethod), Changed<OscMethod>>) {
    for (_, mut osc_method) in query.iter_mut() {
        if let Some(msg) = osc_method.get_message() {
            println!("OSC message received: {:?}", msg)
        }
    }
}
```

See [examples/basic.rs](examples/basic.rs) for a full example.

## Bevy Compatibility

| bevy | bevy_rosc |
|------|-----------|
| 0.8  | 0.2       |
| 0.7  | 0.1       |