use std::sync::Arc;

use winit::window::Window;
use zg_world::{EngineWindowEvents, Resource};

#[derive(Debug, Resource)]
pub struct WindowRes {
    pub window: Arc<Window>,
    pub events: Vec<EngineWindowEvents>,
}

impl WindowRes {
    pub fn new(window: Arc<Window>) -> Self {
        Self {
            window,
            events: Vec::new(),
        }
    }
}
