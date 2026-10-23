use zg_utils::math::Vec3;
use zg_world_macros::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialHandle(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MeshHandle(pub u32);

#[derive(Clone, Copy, Debug, Component)]
pub struct Transform {
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
    };
    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        let position = [x, y, z];
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_uniform_scale(mut self, scale: f32) -> Self {
        self.scale = [scale, scale, scale];
        self
    }

    pub fn with_scale_xyz(mut self, x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        self.scale = [x_scale, y_scale, z_scale];
        self
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn scale(&self) -> Vec3 {
        self.scale
    }
    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }
}

#[derive(Clone, Copy, Debug, Component)]
pub struct MeshComponent(pub MeshHandle);

#[derive(Clone, Copy, Debug, Component)]
pub struct MaterialComponent(pub MaterialHandle);
