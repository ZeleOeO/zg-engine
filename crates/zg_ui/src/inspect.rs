use std::fmt::Debug;
use zg_managers::Assets;

use zg_world::{Commands, Component, Entity, components::MeshComponent};

pub(crate) trait Inspectable {
    const NAME: &'static str;
    fn inspect(&self, assets: &Assets, ui: &mut egui::Ui);
}

pub(crate) fn show_component<T: Component + Debug + Inspectable + 'static>(
    ui: &mut egui::Ui,
    assets: &Assets,
    commands: &Commands,
    entity: Entity,
) {
    // we check to see if an entity has the component if it does, we return the component and we
    // inspect it
    // it skips any component we don't have

    if let Some(component) = commands.get_entity::<(T,)>(entity).0 {
        egui::CollapsingHeader::new(T::NAME)
            .default_open(true)
            .show(ui, |ui| component.inspect(assets, ui));
    }
}

// Mesh
impl Inspectable for MeshComponent {
    const NAME: &'static str = "Mesh";
    fn inspect(&self, assets: &Assets, ui: &mut egui::Ui) {
        let mesh = assets.mesh_manager.get_mesh_data(self.0.0);
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.add_enabled(false, egui::TextEdit::singleline(&mut mesh.name.as_str()));
        });
        ui.horizontal(|ui| {
            ui.label("Vertex Count");
            ui.add_enabled(
                false,
                egui::TextEdit::singleline(&mut mesh.vertex_count.to_string().as_str()),
            );
        });
    }
}

// Material

// Transform

// Camera

// PointLight
