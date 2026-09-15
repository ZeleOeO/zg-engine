use crate::{
    app::app::App,
    graphics::gpu::InternalGraphics,
    managers::{transform::Transform, Assets},
    systems::system_struct::SystemAggregator,
    world::{components::TransformComponent, world::World},
};

fn main() -> anyhow::Result<()> {
    App::new()?.insert_system(show_system).run()
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

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}
