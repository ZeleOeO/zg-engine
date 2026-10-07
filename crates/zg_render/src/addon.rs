use zg_app::Addon;
use zg_world::{
    SystemFunctionExt, SystemSet,
    schedule_label::{Setup, Update},
};

use crate::{
    RenderQueue, WorldRenderer, graphics_render_system, render_items_system,
    render_light_gizmo_system, render_lights_system, setup_world_renderer,
};

pub struct RenderAddon;

#[derive(SystemSet)]
pub struct RenderSet;

impl Addon for RenderAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_resource(RenderQueue::new())
            .add_resource(WorldRenderer::new())
            .add_system(Setup, setup_world_renderer.in_set(RenderSet))
            .add_system(Update, render_items_system.in_set(RenderSet))
            .add_system(Update, render_lights_system.in_set(RenderSet))
            .add_system(Update, render_light_gizmo_system.in_set(RenderSet))
            .add_system(Update, graphics_render_system.in_set(RenderSet));
    }
}
