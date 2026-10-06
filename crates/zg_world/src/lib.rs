extern crate self as zg_world;

mod archetypes;
mod bundle;
mod commands;
pub mod components;
mod events;
mod query;
mod resources;
mod systems;
mod world;

pub use archetypes::Entity;
pub use commands::{Commands, MutWorldCommand};
pub use events::*;
pub use query::Query;
pub use resources::{Res, ResMut, Resource};
pub use systems::*;
pub use world::*;
pub use zg_world_macros::Resource;
