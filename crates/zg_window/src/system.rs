use zg_world::EventReader;

use crate::events::EngineWindowEvents;

// NOTE: this should be pre update
pub fn send_engine_events(event_reader: EventReader<'static, EngineWindowEvents>) {
    let events = event_reader.events;
    event_reader.command_queue.push(Box::new(move |world| {
        for event in events.read() {
            match event {
                EngineWindowEvents::KeyboardInput(keyboard) => {
                    world.write_events(keyboard);
                }
                EngineWindowEvents::MouseMotion(mouse_motion) => {
                    world.write_events(mouse_motion);
                }
            }
        }
    }));
}
