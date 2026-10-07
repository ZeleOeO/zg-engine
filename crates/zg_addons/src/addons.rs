use zg_app::{Addon, AppExit, WindowHandle};
use zg_camera::Camera3DAddon;
use zg_graphics::InternalGraphics;
use zg_input::InputAddon;
use zg_render::RenderAddon;
use zg_time::TimeAddon;
use zg_window::WindowAddon;
use zg_world::{Commands, Res, schedule_label::Setup};

pub struct DefaultAddon;

impl Addon for DefaultAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_addons(WindowAddon)
            .add_resource(AppExit::new())
            .add_addons(RenderAddon)
            .add_addons(TimeAddon)
            .add_addons(InputAddon)
            .add_addons(Camera3DAddon)
            .add_system(Setup, create_graphics_resource);
    }
}

pub fn create_graphics_resource(mut commands: Commands, window_handle: Res<WindowHandle>) {
    let window = window_handle.0.0.clone();
    let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();
    commands.insert_resource(internal_graphics);
}
