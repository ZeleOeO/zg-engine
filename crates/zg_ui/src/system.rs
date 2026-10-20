use crate::{
    editor::editor_ui,
    state::{EditorState, InputFocus, UiFrame},
    ui::EngineUI,
};
use winit::window::CursorGrabMode;
use zg_managers::Assets;
use zg_window::WindowRes;
use zg_world::{Commands, EventReader, RawWinitWindowEvents, Res, ResMut};

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
    commands: Commands,
    assets: Res<Assets>,
) {
    let raw_input = engine_ui.state.take_egui_input(&window.window);
    let full_output = engine_ui.context.run_ui(raw_input, |ui| {
        editor_ui(ui, &mut editor, &commands, &assets);
    });
    engine_ui
        .state
        .handle_platform_output(&window.window, full_output.platform_output);

    let triangle_mesh = engine_ui
        .context
        .tessellate(full_output.shapes, full_output.pixels_per_point);

    ui_frame.textures_delta.clear();
    ui_frame.triangle_mesh = triangle_mesh;
    ui_frame.textures_delta = full_output.textures_delta;
    ui_frame.pixels_per_point = full_output.pixels_per_point;
}

pub fn ui_input_focus(
    engine_ui: Res<EngineUI>,
    mut editor_state: ResMut<EditorState>,
    mut input_focus: ResMut<InputFocus>,
    window_res: Res<WindowRes>,
) {
    input_focus.game_pointer = editor_state.viewport_hovered || editor_state.viewport_dragging;
    input_focus.game_keyboard =
        editor_state.viewport_focused && engine_ui.context.egui_wants_keyboard_input();

    editor_state.camera_look_active = input_focus.game_pointer && editor_state.viewport_focused;

    let window = &window_res.window;
    if editor_state.camera_look_active {
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
    } else {
        window.set_cursor_visible(true);
        window.set_cursor_grab(CursorGrabMode::None).unwrap();
    }
}
