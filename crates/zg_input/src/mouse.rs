use zg_utils::math::Vec2;
use zg_world::Resource;

#[derive(Debug, Resource)]
pub struct MouseMotion {
    pub delta: Vec2,
}

impl MouseMotion {
    pub(crate) fn new() -> Self {
        Self { delta: [0.0, 0.0] }
    }
}
