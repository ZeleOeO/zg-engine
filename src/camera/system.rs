use std::any::TypeId;

use winit::event::DeviceEvent;

use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::camera::camera::Camera;
use crate::camera::camera_controller::handle_key_controller;
use crate::{camera::camera_controller::CameraController, world::world::World};

// This is cute, but this is not how a system looks like
// It needs to get the active camera as a component?

pub fn camera_controller_device_sytem(world: &mut World, event: &DeviceEvent) {
    let mut camera_controller = world.get_mut::<CameraController>();
    match event {
        DeviceEvent::MouseMotion { delta } => {
            camera_controller.handle_mouse(delta.0 as f32, delta.1 as f32);
        }
        _ => {}
    }
}

pub fn camera_update_system(world: &mut World) {
    let components = vec![TypeId::of::<Camera>()];

    let archetype_id = &world.get_archetype_by_type_ids(components).unwrap();
    let archetype = &world.archetypes[archetype_id.archetype_id.0 as usize];

    let camera_column: &Vec<Camera> = archetype.get_column_by_type_id(TypeId::of::<Camera>());
}

pub fn input_system(world: &mut World, window_event: WindowEvent) {
    match window_event {
        WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
            ..
        } => {
            if code == KeyCode::Escape && key_state.is_pressed() {
                // event_loop.exit();
            } else {
                handle_key_controller(
                    &mut world.get_mut::<CameraController>(),
                    code,
                    key_state.is_pressed(),
                );
            }
        }
        _ => {}
    }
}
