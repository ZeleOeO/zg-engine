use crate::math::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MaterialHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct MeshHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vec3,
    // rotation: Quat,
    // pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
        }
    }
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}
