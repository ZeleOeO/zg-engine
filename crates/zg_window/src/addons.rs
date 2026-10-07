use zg_app::Addon;
use zg_world::{
    EngineWindowEvents, SystemFunctionExt, SystemSet,
    schedule_label::{PreUpdate, Setup},
};

use crate::{system::send_engine_events, window::create_window_resource};

pub struct WindowAddon;

#[derive(SystemSet)]
pub enum WindowSet {
    PreUpdate,
}

impl Addon for WindowAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_event::<EngineWindowEvents>()
            .add_system(PreUpdate, send_engine_events.in_set(WindowSet::PreUpdate))
            .add_system(Setup, create_window_resource);
    }
}
