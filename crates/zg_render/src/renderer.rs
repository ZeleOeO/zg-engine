use wgpu::{Buffer, BufferUsages, wgt::BufferDescriptor};

use zg_graphics::InternalGraphics;
use zg_utils::math::Mat4;
use zg_world::Entity;

#[derive(Clone, Debug)]
pub struct WorldRenderer {
    pub default_camera: Option<Entity>,
    pub camera_buffer: Buffer,
}

impl WorldRenderer {
    pub fn new(graphics: &InternalGraphics) -> Self {
        let buffer = graphics.device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<Mat4>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        Self {
            default_camera: None,
            camera_buffer: buffer,
        }
    }
}
