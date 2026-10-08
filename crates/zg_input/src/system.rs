use winit::keyboard::KeyCode;
use zg_app::{Addon, AppExit};
use zg_window::WindowSet;
use zg_world::{
    EventReader, KeyboardInputEvent, MouseMotionEvent, Res, ResMut, SystemFunctionExt, SystemSet,
    schedule_label::{PreUpdate, Update},
};

use crate::{Input, mouse::MouseMotion};

pub fn keyboard_send_event_system(
    mut input_reader: EventReader<KeyboardInputEvent>,
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

pub fn add_mouse_motion_event(
    mut mouse_reader: EventReader<MouseMotionEvent>,
    mut mouse_motion: ResMut<MouseMotion>,
) {
    let mut delta: [f32; 2] = [0.0, 0.0];
    for event in mouse_reader.read() {
        delta[0] += event.delta[0];
        delta[1] += event.delta[1];
    }
    mouse_motion.delta = delta;
}

pub fn check_for_quit(input: Res<Input<KeyCode>>, mut exit: ResMut<AppExit>) {
    if input.is_pressed(KeyCode::Escape) {
        exit.0.0 = true;
    }
}

pub struct InputAddon;

#[derive(SystemSet)]
pub struct InputSet;

impl Addon for InputAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.set_order(WindowSet::PreUpdate, InputSet)
            .add_resource(Input::<KeyCode>::new())
            .add_resource(MouseMotion::new())
            .add_event::<MouseMotionEvent>()
            .add_event::<KeyboardInputEvent>()
            .add_system(PreUpdate, add_mouse_motion_event.in_set(InputSet))
            .add_system(PreUpdate, keyboard_send_event_system.in_set(InputSet))
            .add_system(Update, check_for_quit);
    }
}
