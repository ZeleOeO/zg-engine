use zg_app::{Addon, AppExit, WindowHandle};
use zg_camera::{Camera3DAddon, CameraSet};
use zg_graphics::InternalGraphics;
use zg_input::InputAddon;
use zg_managers::Assets;
use zg_render::{RenderAddon, RenderSet};
use zg_time::TimeAddon;
use zg_window::{WindowAddon, WindowSet};
use zg_world::{Commands, DefaultSet, Res, SystemFunctionExt, SystemSet, schedule_label::Setup};

pub struct DefaultAddon;

impl Addon for DefaultAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.set_order(GraphicsSystemSet, CameraSet::Setup)
            .set_order(GraphicsSystemSet, RenderSet::Setup)
            .set_order(GraphicsSystemSet, WindowSet::PreUpdate)
            .set_order(DefaultSet, RenderSet::Update)
            .add_addons(WindowAddon)
            .add_resource(AppExit::new())
            .add_resource(Assets::new())
            .add_addons(RenderAddon)
            .add_addons(TimeAddon)
            .add_addons(InputAddon)
            .add_addons(Camera3DAddon)
            .add_system(Setup, create_graphics_resource.in_set(GraphicsSystemSet));
    }
}

pub fn create_graphics_resource(mut commands: Commands, window_handle: Res<WindowHandle>) {
    println!("Added graphics");
    let window = window_handle.0.0.clone();
    let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();
    commands.insert_resource(internal_graphics);
}

#[derive(SystemSet)]
struct GraphicsSystemSet;
