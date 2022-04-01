extern crate bevy_osc;

use bevy::prelude::*;

use bevy_osc::{OscMethod, OscMultiMethod};
use bevy_osc::OscDispatcher;
use rosc::{OscBundle, OscMessage, OscTime};
use rosc::OscPacket;
use rosc::OscType;


#[derive(Component)]
struct TestEntity;

#[derive(Component)]
struct TestComponent {
    pub value: i32,
}

#[derive(Bundle)]
#[derive(Component)]
struct TestBundle {
    _t: TestEntity,
    receiver: OscMethod,
    test_component: TestComponent,
}

fn send_single(mut disp: ResMut<OscDispatcher>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    disp.dispatch(vec![OscPacket::Message(OscMessage { addr: "/foo".to_string(), args: vec![1i32.into()] })], method_query, multi_method_query);
}

fn send_wildcard(mut disp: ResMut<OscDispatcher>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    disp.dispatch(vec![OscPacket::Message(OscMessage { addr: "/*/value".to_string(), args: vec![1i32.into()] })], method_query, multi_method_query);
}

fn send_bundle(mut disp: ResMut<OscDispatcher>, method_query: Query<&mut OscMethod>, multi_method_query: Query<&mut OscMultiMethod>) {
    let new_msg = OscBundle {
        timetag: OscTime { seconds: 0, fractional: 0 },
        content: vec![
            OscPacket::Message(OscMessage { addr: "/entity1/value".to_string(), args: vec![1i32.into()] }),
            OscPacket::Bundle(OscBundle {
                timetag: OscTime { seconds: 0, fractional: 0 },
                content: vec![
                    OscPacket::Message(OscMessage { addr: "/entity2/value".to_string(), args: vec![2i32.into()] }),
                    OscPacket::Message(OscMessage { addr: "/entity3/value".to_string(), args: vec![3i32.into()] }),
                ],
            }),
        ],
    };

    disp.dispatch(vec![OscPacket::Bundle(new_msg)], method_query, multi_method_query);
}

fn react_to_message(mut query: Query<(&TestEntity, &mut OscMethod, &mut TestComponent), Changed<OscMethod>>) {
    for (_, mut osc_receiver, mut test_component) in query.iter_mut() {
        let new_msg = osc_receiver.get_message();
        match new_msg {
            Some(msg) => {
                println!("{:?}", msg);
                if msg.args.len() == 1 {
                    match msg.args[0] {
                        OscType::Int(i) => test_component.value = i,
                        _ => {}
                    }
                }
            }
            None => {}
        }
    }
}

#[test]
// Dispatch a single message to a single entity
fn dispatch_osc_message() {
    let mut world = World::default();

    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(send_single.label("send"));
    update_stage.add_system(react_to_message.after("send"));

    world.insert_resource(OscDispatcher::default());

    let test_entity_id = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/foo").expect(""),
    }).id();

    update_stage.run(&mut world);

    assert_eq!(world.get::<TestComponent>(test_entity_id).expect("").value, 1);
}

#[test]
// Dispatch a single message to multiple entities using wildcard
fn dispatch_osc_message_to_multiple_targets() {
    let mut world = World::default();

    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(send_wildcard.label("send"));
    update_stage.add_system(react_to_message.after("send"));

    world.insert_resource(OscDispatcher::default());

    let test_entity_id1 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity1/value").expect(""),
    }).id();
    let test_entity_id2 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity2/value").expect(""),
    }).id();
    let test_entity_id3 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity3/value").expect(""),
    }).id();

    update_stage.run(&mut world);

    assert_eq!(world.get::<TestComponent>(test_entity_id1).expect("").value, 1);
    assert_eq!(world.get::<TestComponent>(test_entity_id2).expect("").value, 1);
    assert_eq!(world.get::<TestComponent>(test_entity_id3).expect("").value, 1);
}

#[test]
// Dispatch a single message to multiple entities using wildcard
fn dispatch_osc_bundle() {
    let mut world = World::default();

    let mut update_stage = SystemStage::parallel();
    update_stage.add_system(send_bundle.label("send"));
    update_stage.add_system(react_to_message.after("send"));

    world.insert_resource(OscDispatcher::default());

    let test_entity_id1 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity1/value").expect(""),
    }).id();
    let test_entity_id2 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity2/value").expect(""),
    }).id();
    let test_entity_id3 = world.spawn().insert_bundle(TestBundle {
        _t: TestEntity,
        test_component: TestComponent { value: 0 },
        receiver: OscMethod::new("/entity3/value").expect(""),
    }).id();

    update_stage.run(&mut world);

    assert_eq!(world.get::<TestComponent>(test_entity_id1).expect("").value, 1);
    assert_eq!(world.get::<TestComponent>(test_entity_id2).expect("").value, 2);
    assert_eq!(world.get::<TestComponent>(test_entity_id3).expect("").value, 3);
}