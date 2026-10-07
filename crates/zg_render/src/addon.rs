use zg_app::Addon;
use zg_world::{SystemFunctionExt, SystemSet, schedule_label::Update};

use crate::{RenderQueue, render_items_system, render_lights_system};

pub struct RenderAddon;
pub struct RenderSet;
impl SystemSet for RenderSet {}

impl Addon for RenderAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_resource(RenderQueue::new())
            .add_system(Update, render_items_system.in_set(RenderSet))
            .add_system(Update, render_lights_system.in_set(RenderSet))
            .add_system(Update, render);
    }
}
