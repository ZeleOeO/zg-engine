use std::sync::Arc;

use winit::event::DeviceEvent;

use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use zg_utils::time::Time;

use crate::camera::Camera;
use crate::camera_controller::{CameraController, handle_key_controller};

use zg_graphics::*;
use zg_render::{FrameBinding, RenderQueue, WorldRenderer, create_camera_bind_group};
use zg_world::{EngineEvents, EventRef, Res, ResMut, Scene, World};

pub fn camera_controller_device_sytem(world: &mut World, event: &DeviceEvent) {
    let mut camera_controller = world.get_mut::<CameraController>();
    match event {
        DeviceEvent::MouseMotion { delta } => {
            camera_controller.handle_mouse(delta.0 as f32, delta.1 as f32);
        }
        _ => {}
    }
}

pub fn camera_controller_system(mut renderer: ResMut<WorldRenderer>, mut scene: Scene) {
    let camera = Camera::default();
    let entity = scene.spawn((camera,));
    renderer.default_camera = Some(entity)
}

pub fn camera_update(
    mut graphics: ResMut<InternalGraphics>,
    mut render_queue: ResMut<RenderQueue>,
    renderer: Res<WorldRenderer>,
    window: Res<Arc<Window>>,
    scene: Scene,
) {
    if let Some(camera_entity) = renderer.default_camera {
        // This changes the aspect ratio for the camera
        let window_size = window.inner_size();
        let camera = scene.get_entity::<(Camera,)>(camera_entity).0;
        camera.aspect = (window_size.width as f32) / window_size.height as f32;

        let view_proj = camera.build_projection_matrix();

        let camera_bind_group_cache_handle =
            create_camera_bind_group(view_proj, camera.eye, &mut graphics, &renderer);

        render_queue.frame_binding.push(FrameBinding {
            bind_group: camera_bind_group_cache_handle,
        });
    }
}

pub fn camera_controller_sytem(
    renderer: Res<WorldRenderer>,
    scene: Scene,
    time: Res<Time>,
    camera_controller: Res<CameraController>,
) {
    let camera_entity = renderer.default_camera.unwrap();
    let camera = scene.get_entity::<(Camera,)>(camera_entity).0;
    let delta = time.time_delta_secs().min(0.1);
    camera_controller.camera_update(camera, delta);
}

pub fn camera_input_system(event: EventRef, mut camera_controller: ResMut<CameraController>) {
    if let EngineEvents(WindowEvent::KeyboardInput {
        event:
            KeyEvent {
                physical_key: PhysicalKey::Code(code),
                state: key_state,
                ..
            },
        ..
    }) = event.0
    {
        if *code == KeyCode::Escape && key_state.is_pressed() {
            event_loop.exit();
        } else {
            handle_key_controller(&mut camera_controller, *code, key_state.is_pressed());
        }
    };
}
pub fn camera_window_event(
    world: &mut World,
    window_event: &WindowEvent,
    event_loop: &ActiveEventLoop,
) {
    let controller = &mut world.get_mut::<CameraController>();
    match window_event {
        WindowEvent::KeyboardInput {
            event:
                KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..
                },
            ..
        } => {
            if *code == KeyCode::Escape && key_state.is_pressed() {
                event_loop.exit();
            } else {
                handle_key_controller(controller, *code, key_state.is_pressed());
            }
        }
        _ => {}
    }
}
