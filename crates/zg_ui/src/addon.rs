use zg_app::{Addon, WindowHandle};
use zg_graphics::InternalGraphics;
use zg_input::InputFocus;
use zg_world::{
    Commands, Res, SystemFunctionExt, SystemSet,
    schedule_label::{PreUpdate, Setup, Update},
};

use crate::{
    EditorState, EngineUI, UiFrame,
    system::{handle_ui_window_events, setup_ui_editor, ui_input_focus, update_ui_frame},
};

pub struct UiAddon;

impl Addon for UiAddon {
    fn build(&self, app: &mut zg_app::App) {
        app.add_resource(UiFrame::new())
            .add_resource(EditorState::new())
            .add_resource(InputFocus::default())
            .add_system(Setup, create_engine_ui.in_set(UiSystemSet::Setup))
            .add_system(Setup, setup_ui_editor.in_set(UiSystemSet::Setup))
            .add_system(PreUpdate, handle_ui_window_events)
            .add_system(Update, update_ui_frame.in_set(UiSystemSet::Update))
            .add_system(Update, ui_input_focus);
    }
}

#[derive(SystemSet)]
pub enum UiSystemSet {
    Setup,
    Update,
}

pub fn create_engine_ui(
    mut commands: Commands,
    window_handle: Res<WindowHandle>,
    internal_graphics: Res<InternalGraphics>,
) {
    let window = window_handle.0.0.clone();
    let engine_ui = EngineUI::new(&internal_graphics, window);
    commands.insert_resource(engine_ui);
}
