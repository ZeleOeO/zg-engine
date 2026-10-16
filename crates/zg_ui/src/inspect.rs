use zg_world::{Commands, Component, Entity, components::MeshComponent};

trait Inspectable {
    fn inspect(&self, ui: &egui::Ui);
}

fn show_component<T: Component + Inspectable>(
    ui: &mut egui::Ui,
    mut commands: Commands,
    entity: Entity,
) {
    // if let Some(component) = scene.try_get::<T>(entity) {
    //     egui::CollapsingHeader::new(T::NAME)
    //         .default_open(true)
    //         .show(ui, |ui| component.inspect(ui, assets));
    // }
}

impl Inspectable for MeshComponent {
    fn inspect(&self, ui: &egui::Ui) {}
}
