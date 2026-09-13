use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use crate::{systems::system_struct::Systems, world::world::World};

pub struct EngineApp {
    pub world: World,
    pub systems: Systems,
    pub window: Option<Arc<Window>>,
}

impl EngineApp {
    pub async fn new() -> Self {
        Self {
            world: World::new(),
            systems: Systems::default(),
            window: None,
        }
    }

    pub fn add_window(&mut self, window: Arc<Window>) {
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        self.window = Some(window)
    }
}
