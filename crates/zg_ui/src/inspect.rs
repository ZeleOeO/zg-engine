use std::fmt::Debug;
use zg_managers::{Assets, PointLight, material::MaterialType};

use zg_utils::math::Vec3;
use zg_world::{
    Commands, Component, Entity,
    components::{Camera, MaterialComponent, MeshComponent, Transform},
};

pub(crate) trait Inspectable {
    const NAME: &'static str;
    fn inspect(&self, assets: &Assets, ui: &mut egui::Ui);
}

pub(crate) fn show_component<T: Component + Debug + Inspectable + 'static>(
    ui: &mut egui::Ui,
    assets: &Assets,
    commands: &Commands,
    entity: &Entity,
) {
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
impl Inspectable for MaterialComponent {
    const NAME: &'static str = "Material";

    fn inspect(&self, assets: &Assets, ui: &mut egui::Ui) {
        let material = assets.material_manager.get_material_data(self.0);

        ui.horizontal(|ui| {
            ui.label("Material Type");

            match &material.material_type {
                MaterialType::Textured { location } => {
                    ui.add(egui::TextEdit::singleline(&mut location.as_str()));
                }
                MaterialType::NonTexture { color } => {
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(40.0, 18.0), egui::Sense::hover());
                    ui.painter().rect_filled(
                        rect,
                        3.0,
                        egui::Color32::from_rgb(
                            (color[0] * 255.0) as u8,
                            (color[1] * 255.0) as u8,
                            (color[2] * 255.0) as u8,
                        ),
                    );
                    ui.label(format!(
                        "({:.2}, {:.2}, {:.2})",
                        color[0], color[1], color[2]
                    ));
                }
            }
        });
    }
}

// Transform
impl Inspectable for Transform {
    const NAME: &'static str = "Transform";

    fn inspect(&self, _assets: &Assets, ui: &mut egui::Ui) {
        let rotation_deg = self.rotation().map(f32::to_degrees);

        egui::Grid::new(ui.id().with("transform_grid"))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                vec3_row(ui, "Position", self.position());
                vec3_row(ui, "Rotation", rotation_deg);
                vec3_row(ui, "Scale", self.scale());
            });
    }
}

fn vec3_row(ui: &mut egui::Ui, label: &str, v: Vec3) {
    const AXES: [&str; 3] = ["x ", "y ", "z "];
    ui.label(label);
    ui.horizontal(|ui| {
        for i in 0..3 {
            ui.label(format!("{}{:.3}", AXES[i], v[i]));
        }
    });
    ui.end_row();
}

impl Inspectable for PointLight {
    const NAME: &'static str = "Point Light";

    fn inspect(&self, _assets: &Assets, ui: &mut egui::Ui) {
        egui::Grid::new(ui.id().with("point_light_grid"))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                vec3_row(ui, "Position", self.position);
                color_row(ui, "Color", self.color);
            });
    }
}

impl Inspectable for Camera {
    const NAME: &'static str = "Camera";

    fn inspect(&self, _assets: &Assets, ui: &mut egui::Ui) {
        egui::Grid::new(ui.id().with("camera_grid"))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                vec3_row(ui, "Eye", self.eye);
                vec3_row(ui, "Target", self.target);
                vec3_row(ui, "Up", self.up);
                scalar_row(ui, "FOV (deg)", self.fovy.to_degrees());
                scalar_row(ui, "Near", self.znear);
                scalar_row(ui, "Far", self.zfar);
                scalar_row(ui, "Aspect", self.aspect);
            });
    }
}

fn scalar_row(ui: &mut egui::Ui, label: &str, value: f32) {
    ui.label(label);
    ui.label(format!("{:.3}", value));
    ui.end_row();
}

fn color_row(ui: &mut egui::Ui, label: &str, c: Vec3) {
    ui.label(label);
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 18.0), egui::Sense::hover());
        let to_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        ui.painter().rect_filled(
            rect,
            3.0,
            egui::Color32::from_rgb(to_u8(c[0]), to_u8(c[1]), to_u8(c[2])),
        );
        ui.label(format!("({:.2}, {:.2}, {:.2})", c[0], c[1], c[2]));
    });
    ui.end_row();
}
