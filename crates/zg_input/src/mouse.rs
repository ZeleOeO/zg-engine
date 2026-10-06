use zg_utils::math::Vec2;
use zg_world::Resource;

#[derive(Debug, Clone)]
pub struct MouseMotionEvent {
    pub delta: Vec2,
}

#[derive(Debug, Resource)]
pub struct MouseMotion {
    pub delta: Vec2,
}
