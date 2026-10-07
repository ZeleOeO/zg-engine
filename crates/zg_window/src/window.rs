use std::sync::Arc;

use winit::window::Window;
use zg_app::WindowHandle;
use zg_world::{Commands, EngineWindowEvents, Res, Resource};

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

pub fn create_window_resoure(window: Res<WindowHandle>, mut commands: Commands) {
    let window_res = WindowRes::new(window.0.0.clone());
    commands.insert_resource(window_res);
}
