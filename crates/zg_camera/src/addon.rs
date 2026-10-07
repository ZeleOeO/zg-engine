use zg_app::Addon;
use zg_world::{
    SystemFunctionExt, SystemSet,
    schedule_label::{Setup, Update},
};

use crate::{CameraController, system::*};

pub struct Camera3DAddon;

#[derive(SystemSet)]
pub enum CameraSet {
    Setup,
    Update,
}

impl Addon for Camera3DAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_resource(CameraController::new(2.0, 0.2))
            .add_system(Setup, camera_setup_system.in_set(CameraSet::Setup))
            .add_system(Update, camera_update_system)
            .add_system(Update, camera_controller_mouse_input_system)
            .add_system(Update, camera_controller_keyboard_system)
            .add_system(Update, camera_controller_udpate_sytem);
    }
}
