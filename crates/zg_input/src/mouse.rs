use zg_utils::math::Vec2;
use zg_world::Resource;

#[derive(Debug, Resource)]
pub struct MouseMotion {
    pub delta: Vec2,
}
