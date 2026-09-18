use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use zg_camera::CameraController;
use zg_graphics::InternalGraphics;
use zg_render::{RenderQueue, WorldRenderer};
use zg_systems::Systems;

use zg_managers::Assets;
use zg_utils::time::Time;
use zg_world::World;

pub(crate) struct EngineApp {
    pub world: World,
    pub systems: Systems,
    pub window: Option<Arc<Window>>,
}

impl EngineApp {
    pub(crate) async fn new() -> Self {
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

    pub(crate) fn insert_default_resources(&mut self, window: Arc<Window>) {
        let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();
        let assets = Assets::new();
        let camera_controller = CameraController::new(2.0, 0.2);
        let render_queue = RenderQueue::default();
        let renderer = WorldRenderer::new(&internal_graphics);
        let time = Time::new();

        self.world.insert(window);
        self.world.insert(internal_graphics);
        self.world.insert(assets);
        self.world.insert(camera_controller);
        self.world.insert(render_queue);
        self.world.insert(renderer);
        self.world.insert(time);
    }
}
