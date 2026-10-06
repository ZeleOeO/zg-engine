use std::fmt::Debug;

use winit::keyboard::KeyCode;

#[derive(Debug, Clone)]
pub struct KeyboardInput {
    pub code: KeyCode,
    pub key_pressed: bool,
}
