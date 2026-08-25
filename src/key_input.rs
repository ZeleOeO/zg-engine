use winit::{
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    camera::camera_controller::{CameraController, handle_key_controller},
    world::world::World,
};

pub fn handle_key_system(
    code: KeyCode,
    is_pressed: bool,
    event_loop: &ActiveEventLoop,
    controller: &mut CameraController,
) {
    if code == KeyCode::Escape && is_pressed {
        event_loop.exit();
    } else {
        handle_key_controller(controller, code, is_pressed);
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
