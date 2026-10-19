use zg_app::WindowHandle;
use zg_world::{Commands, EngineWindowEvents, EventReader, Res, ResMut, ResizeEvent};

use crate::{SceneTargetSize, WindowRes};

pub fn send_engine_events(mut event_reader: EventReader<EngineWindowEvents>) {
    let events_read: Vec<EngineWindowEvents> = event_reader.read().clone();
    event_reader.command_queue.push(Box::new(move |world| {
        for event in events_read {
            match event {
                EngineWindowEvents::KeyboardInput(keyboard) => {
                    world.write_events(keyboard);
                }
                EngineWindowEvents::MouseMotion(mouse_motion) => {
                    world.write_events(mouse_motion);
                }
                EngineWindowEvents::Resize(resize_event) => {
                    world.write_events(resize_event);
                }
            }
        }
    }));
}

pub fn create_window_resource(window: Res<WindowHandle>, mut commands: Commands) {
    let window_res = WindowRes::new(window.0.0.clone());
    commands.insert_resource(window_res);
}

pub fn handle_resize(
    mut scene_size: ResMut<SceneTargetSize>,
    mut resize_event_reader: EventReader<ResizeEvent>,
) {
    if let Some(size) = resize_event_reader.last() {
        scene_size.update(size.0);
    };
}
