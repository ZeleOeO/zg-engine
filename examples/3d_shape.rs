use zg_engine::prelude::*;
use zg_world::{components::Transform, schedule_label::Setup};

fn main() -> anyhow::Result<()> {
    App::new()?
        .add_addons(DefaultAddon)
        .add_system(Setup, create_mesh)
        .run()
}

pub fn create_mesh(
    mut assets: ResMut<Assets>,
    mut graphics: ResMut<InternalGraphics>,
    mut commands: Commands,
) {
    let cube_mesh = assets.create_cube(&graphics);
    let prism_mesh = assets.create_prism(&graphics);
    let tree_material = assets.create_material(
        &mut graphics,
        zg_managers::material::MaterialType::Textured {
            location: "assets/images/happy-tree.png".to_string(),
        },
    );

    println!("Run this command");
    commands.spawn((
        PointLight::new([0.8, 0.5, -0.6], [1.0, 1.0, 1.0]),
        cube_mesh,
    ));
    commands.spawn((
        prism_mesh,
        tree_material,
        Transform::from_translation(3.0, 1.0, 3.0),
    ));
}
