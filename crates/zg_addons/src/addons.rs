use zg_app::{Addon, AppExit};
use zg_camera::Camera3DAddon;
use zg_graphics::InternalGraphics;
use zg_input::InputAddon;
use zg_render::RenderAddon;
use zg_time::TimeAddon;
use zg_window::WindowAddon;

pub struct DefaultAddon;

impl Addon for DefaultAddon {
    fn build(&self, app: &mut zg_app::App) {
        let window = app.window();
        let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();
        app.add_resource(internal_graphics)
            .add_addons(WindowAddon)
            .add_resource(AppExit::new())
            .add_addons(RenderAddon)
            .add_addons(TimeAddon)
            .add_addons(InputAddon)
            .add_addons(Camera3DAddon);
    }
}
