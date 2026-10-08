use std::fmt::Debug;
use zg_world_macros::Resource;

#[derive(Debug, Resource)]
pub struct Events<E: Debug + Clone + 'static> {
    events: Vec<E>,
}

impl<E: Debug + Clone> Events<E> {
    pub fn new() -> Events<E> {
        let events: Vec<E> = Vec::new();
        Events { events }
    }

    pub fn write(&mut self, event: E) {
        self.events.push(event);
    }

    pub fn read(&mut self) -> Vec<E> {
        self.events.drain(..).collect()
    }

    pub fn write_batch(&mut self, events: Vec<E>) {
        for event in events {
            self.write(event);
        }
    }
}
