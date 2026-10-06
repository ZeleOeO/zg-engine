use winit::keyboard::KeyCode;
use zg_world::{EventReader, KeyboardInputEvent, MouseMotionEvent, ResMut};

use crate::{Input, mouse::MouseMotion};

// NOTE: keyboard input runs preupdate
pub fn keyboard_send_event_system(
    input_reader: EventReader<KeyboardInputEvent>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    for event in input_reader.read() {
        let input = event.code;
        if event.key_pressed {
            keyboard_input.press(input);
        } else {
            keyboard_input.release(input);
        }
    }
}

// NOTE: runs preupdate
pub fn add_mouse_motion_event(
    mouse_reader: EventReader<MouseMotionEvent>,
    mut mouse_motion: ResMut<MouseMotion>,
) {
    let mut delta: [f32; 2] = [0.0, 0.0];
    for event in mouse_reader.read() {
        delta[0] += event.delta[0];
        delta[1] += event.delta[1];
    }
    mouse_motion.delta = delta;
}
