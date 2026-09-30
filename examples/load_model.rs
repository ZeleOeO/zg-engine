use zg_engine::prelude::*;

fn main() -> anyhow::Result<()> {
    App::new()?.insert_system(show_system).run()
}

pub fn instantiate_mesh(world: &mut World) {
    let chicken = Model::load_obj(world, "assets/obj/animals_obj/chicken_001.obj").unwrap();

    let graphics = world.get_mut::<InternalGraphics>();
    let mut assets = world.get_mut::<Assets>();

    let cube_mesh = assets.create_cube(&graphics);

    drop(graphics);
    drop(assets);

    world.spawn((
        PointLight::new([0.8, 0.5, -0.6], [1.0, 1.0, 1.0]),
        cube_mesh,
    ));
    println!("Spawned object as entities: {:?}", chicken);
}

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}
