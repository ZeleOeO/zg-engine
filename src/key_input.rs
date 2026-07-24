use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::camera::camera_controller::{CameraController, handle_key_controller};

pub fn handle_key(
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
