use zg_input::{KeyboardInput, MouseMotionEvent};

// impl EventReader {}
// world.send_event(e)
// which would get the Event Resource and push it into the world
//

#[derive(Debug, Clone)]
pub enum EngineWindowEvents {
    KeyboardInput(KeyboardInput),
    MouseMotion(MouseMotionEvent),
}
