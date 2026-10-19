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

impl SceneTargetSize {
    pub fn new() -> SceneTargetSize {
        SceneTargetSize([1280, 720])
    }

    pub fn update(&mut self, new_size: [u32; 2]) {
        self.0 = new_size
    }
}

#[derive(Debug, Resource)]
pub struct SceneTargetSize(pub [u32; 2]);
