use crate::app::app::App;

pub mod app;
pub mod camera;
pub mod graphics;
pub mod layouts;
pub mod managers;
pub mod math;
pub mod pipeline;
pub mod render;
pub mod systems;
pub mod utils;
pub mod world;

fn main() -> anyhow::Result<()> {
    let app = App::new();
    Ok(())
}
