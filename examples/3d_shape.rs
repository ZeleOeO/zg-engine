use zg_engine::prelude::*;

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
        zg_managers::material::MaterialType::Textured {
            location: "assets/images/happy-tree.png".to_string(),
        },
    );
    let color_mat = assets.create_material(
        &mut graphics,
        zg_managers::material::MaterialType::NonTexture {
            color: [0.0, 0.0, 1.0],
        },
    );

    drop(assets);
    drop(graphics);

    // I could get the vector of the typeId interestingly
    world.spawn((cube_mesh, tree_material, Transform::IDENTITY));
    world.spawn((
        prism_mesh,
        color_mat,
        Transform::from_translation(1.0, 2.0, 3.0),
    ));
    world.spawn((
        cube_mesh,
        color_mat,
        Transform::from_translation(8.0, 2.0, 3.0).with_scale(4.0, 3.0, 4.0),
    ));
}

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}
