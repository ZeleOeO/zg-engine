use crate::{
    graphics::gpu::InternalGraphics, managers::Manager, render::command::RenderCommand,
    world::world::World,
};

#[derive(Default)]
pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}

impl RenderQueue {
    pub fn flush(&mut self, render_pass: &mut wgpu::RenderPass, world: &World) {
        let graphics = world.get::<InternalGraphics>();
        let manager = world.get::<Manager>();
        for command in self.commands.drain(..) {
            match command {
                RenderCommand::SetVertexBuffer { mesh_handle } => {
                    let buffer = manager.mesh_manager.get_vertex_buffers(mesh_handle.0);
                    render_pass.set_vertex_buffer(0, buffer.slice(..));
                }
                RenderCommand::SetIndexBuffer { index_handle } => {
                    let buffer = manager.mesh_manager.get_index_buffers(index_handle.0);
                    render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint32);
                }
                RenderCommand::SetPipeline { pipeline_id } => {
                    let pipeline = graphics.get_pipeline_cache(pipeline_id);
                    render_pass.set_pipeline(pipeline);
                }
                RenderCommand::SetBindGroup { bind_group_handle } => {
                    let bind_group_cached = graphics.get_bind_group_by_handle(bind_group_handle);
                    render_pass.set_bind_group(bind_group_handle.1, bind_group_cached, &[]);
                }
                RenderCommand::DrawIndexed { num_to_draw } => {
                    render_pass.draw_indexed(0..num_to_draw, 0, 0..1);
                }
            }
        }
    }
}
