use zg_engine::prelude::*;

fn main() -> anyhow::Result<()> {
    App::new()?.insert_system(show_system).run()
}

pub fn instantiate_mesh(world: &mut World) {
    let mut graphics = world.get_mut::<InternalGraphics>();
    let mut assets = world.get_mut::<Assets>();

    let obj_mesh = assets
        .load_obj_model(&graphics, "assets/obj/test.obj")
        .unwrap();

    let color_mat = assets.create_material(
        &mut graphics,
        zg_managers::material::MaterialType::NonTexture {
            color: [0.3, 0.7, 1.0],
        },
    );

    // to prevent this, I might need to make a "scene" type that will handle the ecs stuff

    drop(assets);
    drop(graphics);

    world.spawn((obj_mesh, TransformComponent::default(), color_mat));
}

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}
