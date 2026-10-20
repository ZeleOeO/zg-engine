use winit::keyboard::KeyCode;
use zg_input::{Input, InputFocus, MouseMotion};
use zg_time::Time;
use zg_window::SceneTargetSize;

use crate::camera::Camera;
use crate::camera_controller::CameraController;

use zg_graphics::*;
use zg_render::{FrameBinding, RenderQueue, WorldRenderer, create_camera_bind_group};
use zg_world::{Commands, Res, ResMut};

pub fn camera_controller_mouse_input_system(
    mouse_motion: ResMut<MouseMotion>,
    mut camera_controller: ResMut<CameraController>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.camera_look_active {
        camera_controller.handle_mouse(mouse_motion.delta[0], mouse_motion.delta[1]);
    }
}

pub fn camera_setup_system(mut renderer: ResMut<WorldRenderer>, mut scene: Commands) {
    let camera = Camera::default();
    let entity = scene.spawn((camera,));
    renderer.default_camera = Some(entity)
}

pub fn camera_update_system(
    mut graphics: ResMut<InternalGraphics>,
    mut render_queue: ResMut<RenderQueue>,
    renderer: Res<WorldRenderer>,
    scene_size: Res<SceneTargetSize>,
    command: Commands,
) {
    if let Some(camera_entity) = renderer.default_camera.clone() {
        if let Some(camera) = command.get_entity::<(Camera,)>(camera_entity).0 {
            let [width, height] = scene_size.0.0;
            camera.aspect = (width as f32) / height as f32;

            let view_proj = camera.build_projection_matrix();

            let camera_bind_group_cache_handle =
                create_camera_bind_group(view_proj, camera.eye, &mut graphics.0, &renderer.0);

            render_queue.frame_binding.push(FrameBinding {
                bind_group: camera_bind_group_cache_handle,
            });
        };
    }
}

pub fn camera_controller_udpate_sytem(
    renderer: Res<WorldRenderer>,
    scene: Commands,
    time: Res<Time>,
    camera_controller: Res<CameraController>,
) {
    let camera_entity = renderer.default_camera.clone().unwrap();
    let delta = time.time_delta_secs().min(0.1);
    if let Some(camera) = scene.get_entity::<(Camera,)>(camera_entity).0 {
        camera_controller.camera_update(camera, delta);
    }
}

pub fn camera_controller_keyboard_system(
    mut camera_controller: ResMut<CameraController>,
    input: Res<Input<KeyCode>>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.game_keyboard {
        camera_controller.is_forward_key_pressed =
            input.is_pressed(KeyCode::ArrowUp) || input.is_pressed(KeyCode::KeyW);
        camera_controller.is_backward_key_pressed =
            input.is_pressed(KeyCode::ArrowDown) || input.is_pressed(KeyCode::KeyS);
        camera_controller.is_left_key_pressed =
            input.is_pressed(KeyCode::ArrowLeft) || input.is_pressed(KeyCode::KeyA);
        camera_controller.is_right_key_pressed =
            input.is_pressed(KeyCode::ArrowRight) || input.is_pressed(KeyCode::KeyD);
    } else {
        camera_controller.is_forward_key_pressed = false;
        camera_controller.is_backward_key_pressed = false;
        camera_controller.is_left_key_pressed = false;
        camera_controller.is_right_key_pressed = false;
    }
}
