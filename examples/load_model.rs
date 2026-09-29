use zg_engine::prelude::*;

fn main() -> anyhow::Result<()> {
    App::new()?.insert_system(show_system).run()
}

pub fn instantiate_mesh(world: &mut World) {
    let chicken = Model::load_obj(world, "assets/obj/animals_obj/chicken_001.obj").unwrap();

    world.spawn((PointLight::new([-2.0, 1.0, 1.0], [1.0, 1.0, 1.0]),));
    println!("Spawned object as entities: {:?}", chicken);
}

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}
