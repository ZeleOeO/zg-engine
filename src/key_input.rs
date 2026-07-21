use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

pub fn handle_key(code: KeyCode, is_pressed: bool, event_loop: &ActiveEventLoop) {
    match (code, is_pressed) {
        (KeyCode::Escape, true) => {
            event_loop.exit();
        }
        _ => {}
    }
}
