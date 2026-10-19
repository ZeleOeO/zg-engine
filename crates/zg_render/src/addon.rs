use zg_app::Addon;
use zg_window::WindowSet;
use zg_world::{
    SystemFunctionExt, SystemSet,
    schedule_label::{PreUpdate, Setup, Update},
};

use crate::{
    RenderQueue, WorldRenderer, graphics_render_system, render_items_system,
    render_light_gizmo_system, render_lights_system, setup_world_renderer, viewport_resize_system,
};

pub struct RenderAddon;

#[derive(SystemSet)]
pub enum RenderSet {
    Setup,
    PreUpdate,
    Update,
}

impl Addon for RenderAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.set_order(WindowSet::PreUpdate, RenderSet::PreUpdate)
            .add_resource(RenderQueue::new())
            .add_resource(WorldRenderer::new())
            .add_system(Setup, setup_world_renderer.in_set(RenderSet::Setup))
            .add_system(
                PreUpdate,
                viewport_resize_system.in_set(RenderSet::PreUpdate),
            )
            .add_system(Update, render_items_system.in_set(RenderSet::Update))
            .add_system(Update, render_lights_system.in_set(RenderSet::Update))
            .add_system(Update, render_light_gizmo_system.in_set(RenderSet::Update))
            .add_system(Update, graphics_render_system.in_set(RenderSet::Update));
    }
}
