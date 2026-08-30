use std::sync::Arc;
use winit::{
    event::{DeviceEvent, WindowEvent},
    window::{CursorGrabMode, Window},
};

use crate::{
    systems::sysem_struct::{SystemSchedule, Systems},
    world::world::World,
};

pub struct EngineApp {
    pub world: World,
    pub systems: Systems,
    pub window: Arc<Window>,
}

impl EngineApp {
    pub async fn new(window: Arc<Window>) -> Self {
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        Self {
            world: World::new(),
            systems: Systems::default(),
            window: window,
        }
    }
}
