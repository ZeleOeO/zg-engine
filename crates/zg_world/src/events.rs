use winit::keyboard::KeyCode;

use crate::{SystemContext, SystemParam};

#[derive(Debug)]
pub enum EngineEvents {
    KeyboardInput(KeyboardInput),
}

#[derive(Debug)]
pub struct KeyboardInput {
    pub code: KeyCode,
    pub key_status: bool,
}

// pub struct EventRef<'w>(pub &'w EngineEvents);

// impl SystemParam for EventRef<'_> {
//     type Item<'w> = EventRef<'w>;
//     fn extract_world_context<'w>(context: &'w SystemContext) -> Self::Item<'w> {
//         EventRef(context.event)
//     }
// }

// choosing structs cause I don't enjoy enum comparison syntax
//

// pub struct
