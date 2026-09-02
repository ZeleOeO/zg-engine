use winit::keyboard::KeyCode;

use crate::{
    camera::camera::Camera,
    math::{vec3_add, vec3_cross_product, vec3_mult_scal, vec3_normalize, vec3_sub},
};

#[derive(Clone, Debug)]
pub struct CameraController {
    pub speed: f32,
    pub sensitivity: f32,
    pub yaw: f32,
    pub pitch: f32,
    is_forward_key_pressed: bool,
    is_backward_key_pressed: bool,
    is_left_key_pressed: bool,
    is_right_key_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> CameraController {
        Self {
            speed,
            sensitivity,
            yaw: 0.0,
            pitch: 0.0,
            is_forward_key_pressed: false,
            is_backward_key_pressed: false,
            is_left_key_pressed: false,
            is_right_key_pressed: false,
        }
    }

    pub fn handle_mouse(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw -= delta_x.to_radians() * self.sensitivity;
        self.pitch += delta_y.to_radians() * self.sensitivity;
        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    pub fn camera_update(&self, camera: &mut Camera, dt: f32) {
        let sin_yaw = self.yaw.sin();
        let cos_yaw = self.yaw.cos();
        let sin_pitch = self.pitch.sin();
        let cos_pitch = self.pitch.cos();

        let forward = vec3_normalize([cos_pitch * sin_yaw, sin_pitch, cos_pitch * cos_yaw]);
        let right = vec3_normalize(vec3_cross_product(forward, camera.up));
        let speed = self.speed as f32 * dt;

        let mut movement = [0.0, 0.0, 0.0];

        if self.is_forward_key_pressed {
            movement = vec3_add(movement, vec3_mult_scal(forward, speed));
        }
        if self.is_backward_key_pressed {
            movement = vec3_sub(movement, vec3_mult_scal(forward, speed));
        }
        if self.is_right_key_pressed {
            movement = vec3_add(movement, vec3_mult_scal(right, speed));
        }
        if self.is_left_key_pressed {
            movement = vec3_sub(movement, vec3_mult_scal(right, speed));
        }

        camera.eye = vec3_add(camera.eye, movement);
        camera.target = vec3_add(camera.eye, forward);
    }
}

pub fn handle_key_controller(controller: &mut CameraController, code: KeyCode, is_pressed: bool) {
    match code {
        KeyCode::ArrowUp | KeyCode::KeyW => {
            controller.is_forward_key_pressed = is_pressed;
        }

        KeyCode::ArrowDown | KeyCode::KeyS => {
            controller.is_backward_key_pressed = is_pressed;
        }

        KeyCode::ArrowLeft | KeyCode::KeyA => {
            controller.is_left_key_pressed = is_pressed;
        }

        KeyCode::ArrowRight | KeyCode::KeyD => {
            controller.is_right_key_pressed = is_pressed;
        }
        _ => {}
    }
}
