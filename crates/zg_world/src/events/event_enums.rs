use winit::keyboard::KeyCode;
use zg_utils::math::Vec2;

#[derive(Debug, Clone)]
pub struct KeyboardInputEvent {
    pub code: KeyCode,
    pub key_pressed: bool,
}

#[derive(Debug, Clone)]
pub struct MouseMotionEvent {
    pub delta: Vec2,
}

#[derive(Debug, Clone)]
pub enum EngineWindowEvents {
    KeyboardInput(KeyboardInputEvent),
    MouseMotion(MouseMotionEvent),
}
