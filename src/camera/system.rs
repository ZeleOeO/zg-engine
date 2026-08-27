use winit::event::DeviceEvent;

use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::camera::camera_controller::handle_key_controller;
use crate::{camera::camera_controller::CameraController, world::world::World};

pub fn camera_controller_update_system(world: &mut World, event: &DeviceEvent) {
    let mut camera_controller = world.get_mut::<CameraController>();
    match event {
        DeviceEvent::MouseMotion { delta } => {
            camera_controller.handle_mouse(delta.0 as f32, delta.1 as f32);
        }
        _ => {}
    }
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
