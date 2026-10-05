use std::sync::Arc;

use winit::window::Window;
use zg_world::Resource;

#[derive(Debug, Resource)]
pub struct WindowRes {
    window: Arc<Window>,
}

impl WindowRes {
    pub fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}
