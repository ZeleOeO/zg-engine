use zg_world::{EngineWindowEvents, EventReader};

pub fn send_engine_events(mut event_reader: EventReader<EngineWindowEvents>) {
    let events_read: Vec<EngineWindowEvents> = event_reader.events.read().clone();
    event_reader.command_queue.push(Box::new(move |world| {
        for event in events_read {
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
