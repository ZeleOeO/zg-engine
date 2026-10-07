use zg_app::Addon;
use zg_world::{EngineWindowEvents, SystemFunctionExt, SystemSet, schedule_label::PreUpdate};

use crate::{WindowRes, system::send_engine_events};

pub struct WindowAddon;

#[derive(SystemSet)]
pub enum WindowSet {
    PreUpdate,
}

impl Addon for WindowAddon {
    fn build(&self, app: &mut zg_app::App) {
        let window = app.window();
        app.add_resource(WindowRes::new(window))
            .add_event::<EngineWindowEvents>()
            .add_system(PreUpdate, send_engine_events.in_set(WindowSet::PreUpdate));
    }
}
