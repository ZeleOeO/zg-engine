use winit::event::{DeviceEvent, WindowEvent};

use crate::{SystemContext, SystemParam};

#[derive(Debug)]
pub enum EngineEvents {
    Window(WindowEvent),
    Device(DeviceEvent),
}

pub struct EventRef<'w>(pub &'w EngineEvents);

impl SystemParam for EventRef<'_> {
    type Item<'w> = EventRef<'w>;
    fn extract_world_context<'w>(context: &'w SystemContext) -> Self::Item<'w> {
        EventRef(context.event)
    }
}
