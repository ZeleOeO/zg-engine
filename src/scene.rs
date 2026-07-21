use std::rc::Rc;

use wgpu::{Queue, RenderPipeline};

use crate::resources::{
    material::{Material, Uniform},
    mesh::Mesh,
};

#[derive(Clone)]
pub struct DrawItem {
    pub pipeline: Rc<RenderPipeline>,
    pub mesh: Rc<Mesh>,
    pub material: Rc<Material>,
    pub uniform: Uniform,
}

pub struct Scene {
    pub draw_items: Vec<DrawItem>,
}

impl DrawItem {
    pub fn rotate_item(&mut self, queue: &Queue, rotation_number: f32) {
        self.uniform.rotation += rotation_number;
        queue.write_buffer(
            &self.material.uniform_buffer,
            0,
            bytemuck::bytes_of(&self.uniform.rotation),
        );
    }
}
