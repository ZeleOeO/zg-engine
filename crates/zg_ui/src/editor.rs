use egui::Ui;
use zg_managers::Assets;
use zg_world::{Commands, Entity, components::MeshComponent};

use crate::{EditorState, inspect::show_component};

pub(crate) fn editor_ui(
    ui: &mut egui::Ui,
    editor: &mut EditorState,
    commands: &Commands,
    assets: &Assets,
) {
    let entities = commands.get_all_entities();
    egui::Panel::left("scene").show(ui, |ui| {
        ui.heading("Scene");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for entity in entities {
                let is_selected = editor
                    .selected_entities
                    .as_ref()
                    .map_or(false, |edit_entity| edit_entity == entity);

                if ui
                    .selectable_label(is_selected, entity.name.clone())
                    .clicked()
                {
                    editor.selected_entities = Some(entity.clone());
                }
            }
        });
        // entity that's it
    });
    egui::Panel::right("Inspector").show(ui, |ui| {
        let Some(entity) = &editor.selected_entities else {
            ui.label("Nothing is selected");
            return;
        };

        inspect_entity(ui, assets, commands, entity.clone());
    });

    egui::CentralPanel::default().show(ui, |ui| {
        let size = ui.available_size();
        let response = ui.add(
            egui::Image::new((editor.viewport_tex_id, size)).sense(egui::Sense::click_and_drag()),
        );

        if response.clicked() || response.drag_started() {
            response.request_focus();
        }

        ui.memory_mut(|m| {
            m.set_focus_lock_filter(
                response.id,
                egui::EventFilter {
                    horizontal_arrows: true,
                    vertical_arrows: true,
                    tab: false,
                    escape: false,
                },
            );
        });

        let ppp = ui.ctx().pixels_per_point();
        editor.viewport_rect = response.rect;
        editor.viewport_wanted_px = [
            (size.x * ppp).max(1.0) as u32,
            (size.y * ppp).max(1.0) as u32,
        ];
        editor.viewport_hovered = response.hovered();
        editor.viewport_focused = response.has_focus();
        editor.viewport_dragging = response.dragged();
        editor.camera_look_active = response.dragged_by(egui::PointerButton::Secondary);
    });
}

pub(crate) fn inspect_entity(ui: &mut Ui, assets: &Assets, commands: &Commands, entity: Entity) {
    // show_component::<MeshComponent>(ui, assets, commands, entity);
    show_component::<MeshComponent>(ui, assets, commands, entity);
}
