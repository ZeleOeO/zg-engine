extern crate self as zg_world;

mod archetypes;
mod bundle;
pub mod components;
mod events;
mod resources;
mod systems;
mod world;

pub use archetypes::Entity;
pub use events::*;
pub use resources::{Res, ResMut, Resource};
pub use systems::*;
pub use world::*;
pub use zg_world_macros::Resource;
