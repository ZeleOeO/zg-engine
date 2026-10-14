use crate::{
    state::{EditorState, InputFocus, UiFrame},
    ui::EngineUI,
};
use zg_window::WindowRes;
use zg_world::{EventReader, RawWinitWindowEvents, Res, ResMut};

pub fn setup_ui_editor(mut editor_state: ResMut<EditorState>, engine_ui: Res<EngineUI>) {
    editor_state.viewport_tex_id = engine_ui.target_viewport_tex_id;
}

pub fn handle_ui_window_events(
    mut engine_ui: ResMut<EngineUI>,
    window: Res<WindowRes>,
    mut event_reader: EventReader<RawWinitWindowEvents>,
) {
    for event in event_reader.read() {
        let _ = engine_ui
            .state
            .on_window_event(&window.window, &event.event);
    }
}

pub fn update_ui_frame(
    mut engine_ui: ResMut<EngineUI>,
    window: Res<WindowRes>,
    mut ui_frame: ResMut<UiFrame>,
    mut editor: ResMut<EditorState>,
) {
    let raw_input = engine_ui.state.take_egui_input(&window.window);
    let full_output = engine_ui
        .context
        .run_ui(raw_input, |ui| editor_ui(ui, &mut editor));
    engine_ui
        .state
        .handle_platform_output(&window.window, full_output.platform_output);

    let triangle_mesh = engine_ui
        .context
        .tessellate(full_output.shapes, full_output.pixels_per_point);

    ui_frame.triangle_mesh = triangle_mesh;
    ui_frame.textures_delta = full_output.textures_delta;
    ui_frame.pixels_per_point = full_output.pixels_per_point;
}

pub fn ui_input_focus(
    engine_ui: Res<EngineUI>,
    editor_state: Res<EditorState>,
    mut input_focus: ResMut<InputFocus>,
) {
    input_focus.game_pointer = editor_state.viewport_hovered || editor_state.viewport_dragging;
    input_focus.game_keyboard =
        editor_state.viewport_focused && !engine_ui.context.egui_wants_keyboard_input();
}

fn editor_ui(
    ui: &mut egui::Ui,
    editor: &mut EditorState,
    /*world stuff*/
) {
    egui::Panel::left("entities").show(ui, |ui| {
        // entity that's it
    });
    egui::Panel::right("components").show(ui, |ui| {
        // entity name
        // list of entity components
        // for each component the attributes like transform, x, y,z
        // mesh, a drop down maybe of the mesh
        // material, a drop down of that
        // light: type of light, point light etc
        // possibly editable
    });

    egui::CentralPanel::default().show(ui, |ui| {
        let size = ui.available_size();
        let response = ui.add(
            egui::Image::new((editor.viewport_tex_id, size)).sense(egui::Sense::click_and_drag()),
        );

        if response.clicked() || response.drag_started() {
            response.request_focus();
        }

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
