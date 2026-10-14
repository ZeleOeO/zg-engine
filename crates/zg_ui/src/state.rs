use egui::TextureId;
use zg_world::Resource;

#[derive(Debug, Resource)]
pub struct EditorState {
    pub viewport_tex_id: egui::TextureId,
    pub viewport_rect: egui::Rect,
    pub viewport_wanted_px: [u32; 2],
    pub viewport_hovered: bool,
    pub viewport_focused: bool,
    pub viewport_dragging: bool,
    pub camera_look_active: bool,
}

#[derive(Default, Debug, Resource)]
pub struct InputFocus {
    pub game_pointer: bool,
    pub game_keyboard: bool,
}

#[derive(Debug, Resource)]
pub struct UiFrame {
    pub triangle_mesh: Vec<egui::ClippedPrimitive>,
    pub textures_delta: egui::TexturesDelta,
    pub pixels_per_point: f32,
}

impl EditorState {
    pub fn new() -> EditorState {
        EditorState {
            viewport_tex_id: TextureId::default(),
            viewport_rect: egui::Rect::NOTHING,
            viewport_wanted_px: [1280, 720],
            viewport_hovered: false,
            viewport_focused: false,
            viewport_dragging: false,
            camera_look_active: false,
        }
    }
}

impl UiFrame {
    pub fn new() -> Self {
        Self {
            triangle_mesh: Vec::new(),
            textures_delta: egui::TexturesDelta::default(),
            pixels_per_point: 1.0,
        }
    }
}
