use zg_engine::prelude::*;
use zg_world::components::Transform;

fn main() -> anyhow::Result<()> {
    App::new()?.add_addons(DefaultAddon).run()
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

    commands.spawn((
        PointLight::new([0.8, 0.5, -0.6], [1.0, 1.0, 1.0]),
        cube_mesh,
    ));

    // let color_mat = assets.create_material(
    //     &mut graphics,
    //     zg_managers::material::MaterialType::NonTexture {
    //         color: [0.0, 0.0, 1.0],
    //     },
    // );

    commands.spawn((
        prism_mesh,
        tree_material,
        Transform::from_translation(3.0, 1.0, 3.0),
    ));
    // commands.spawn(bundle)
}

// pub fn instantiate_mesh(world: &mut World) {
//     let mut graphics = world.get_resource_mut::<InternalGraphics>();
//     let mut assets = world.get_resource_mut::<Assets>();
//
//     let cube_mesh = assets.create_cube(&graphics);
//     let prism_mesh = assets.create_prism(&graphics);
//     let tree_material = assets.create_material(
//         &mut graphics,
//         zg_managers::material::MaterialType::Textured {
//             location: "assets/images/happy-tree.png".to_string(),
//         },
//     );
//     let color_mat = assets.create_material(
//         &mut graphics,
//         zg_managers::material::MaterialType::NonTexture {
//             color: [0.0, 0.0, 1.0],
//         },
//     );
//
//     drop(assets);
//     drop(graphics);
//
//     // I could get the vector of the typeId interestingly
//
//     world.spawn((prism_mesh, tree_material, Transform::IDENTITY));
//     world.spawn((
//         prism_mesh,
//         color_mat,
//         Transform::from_translation(1.0, 2.0, 3.0),
//     ));
//     world.spawn((
//         cube_mesh,
//         color_mat,
//         Transform::from_translation(8.0, 2.0, 3.0).with_uniform_scale(4.0),
//     ));
// }
