use zg_world::{EngineEvents, SystemParam};

// This is a system param
// I can either make it hold everything or just one particular event
pub struct EventReader<'e> {
    events: &'e Vec<EngineEvents>,
}

// This is a resource
pub struct Events<E> {
    events: Vec<E>,
}

impl SystemParam for EventReader {
    type Item<'w> = EventReader;
    fn extract_world_context<'w>(context: &'w zg_world::SystemContext) -> Self::Item<'w> {
        EventReader {
            events: &context.event_queue,
        }
    }
}

impl EventReader {}
