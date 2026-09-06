use crate::{
    app::app::App,
    graphics::gpu::InternalGraphics,
    managers::{Assets, transform::Transform},
    world::{components::TransformComponent, world::World},
};

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
    App::new()?.add_setup_system(instantiate_mesh).run()
}

pub fn instantiate_mesh(world: &mut World) {
    let mut graphics = world.get_mut::<InternalGraphics>();
    let mut assets = world.get_mut::<Assets>();

    let cube_mesh = assets.create_cube(&graphics);
    let prism_mesh = assets.create_prism(&graphics);
    let tree_material = assets.create_material(
        &mut graphics,
        managers::material::MaterialType::Textured {
            location: "src/assets/happy-tree.png".to_string(),
        },
    );
    let color_mat = assets.create_material(
        &mut graphics,
        managers::material::MaterialType::NonTexture {
            color: [0.0, 0.0, 1.0],
        },
    );

    drop(assets);
    drop(graphics);

    // I could get the vector of the typeId interestingly
    world.spawn((cube_mesh, tree_material, TransformComponent::default()));
    world.spawn((
        prism_mesh,
        color_mat,
        TransformComponent(Transform::new([1.0, 2.0, 3.0])),
    ));
    world.spawn((
        cube_mesh,
        color_mat,
        TransformComponent(Transform::new([8.0, 2.0, 3.0])),
    ));
}
