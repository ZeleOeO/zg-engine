use zg_app::Addon;
use zg_ui::UiSystemSet;
use zg_window::WindowSet;
use zg_world::{
    SystemFunctionExt, SystemSet,
    schedule_label::{PreUpdate, Setup, Update},
};

use crate::{
    RenderQueue, WorldRenderer,
    compute::{
        ComputeQueue, RandomTestStruct, compute_setup_test_sytem, compute_system,
        do_some_compute_stuff,
    },
    graphics_render_system, render_items_system, render_light_gizmo_system, render_lights_system,
    setup_world_renderer, viewport_resize_system,
};

pub struct RenderAddon;

#[derive(SystemSet)]
pub enum RenderSet {
    Setup,
    PreUpdate,
    Update,
}

#[derive(SystemSet)]
enum ComputeSet {
    Setup,
    EncoderSend,
    TestComputePipeline,
}

impl Addon for RenderAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.set_order(WindowSet::PreUpdate, RenderSet::PreUpdate)
            .add_resource(ComputeQueue::default())
            .add_resource(RandomTestStruct::default())
            .set_order(UiSystemSet::PreUpdate, RenderSet::PreUpdate)
            .set_order(ComputeSet::Setup, ComputeSet::EncoderSend)
            .set_order(ComputeSet::EncoderSend, ComputeSet::TestComputePipeline)
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
            .add_system(Update, graphics_render_system.in_set(RenderSet::Update))
            .add_system(Update, compute_setup_test_sytem.in_set(ComputeSet::Setup))
            .add_system(Update, compute_system.in_set(ComputeSet::EncoderSend))
            .add_system(
                Update,
                do_some_compute_stuff.in_set(ComputeSet::TestComputePipeline),
            );
    }
}
