use wgpu::Buffer;

use zg_world::{Entity, Resource};

#[derive(Clone, Debug, Resource)]
pub struct WorldRenderer {
    pub default_camera: Option<Entity>,
    pub camera_buffer: Option<Buffer>,
}

impl WorldRenderer {
    pub fn new() -> Self {
        Self {
            default_camera: None,
            camera_buffer: None,
        }
    }

    pub fn camera_buffer(&self) -> &Buffer {
        self.camera_buffer.as_ref().unwrap()
    }
}

#[derive(Debug, Resource)]
pub struct SceneTargetSize(pub [u32; 2]);

impl SceneTargetSize {
    pub fn new() -> SceneTargetSize {
        SceneTargetSize([1280, 720])
    }
}
