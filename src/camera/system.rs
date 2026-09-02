use std::any::TypeId;
use std::sync::Arc;

use winit::event::DeviceEvent;

use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::camera::camera::Camera;
use crate::camera::camera_controller::handle_key_controller;
use crate::graphics::gpu::InternalGraphics;
use crate::render::buffer::{BindGroupCacheKey, BindGroupResourceType};
use crate::render::command::RenderCommand;
use crate::render::render_queue::RenderQueue;
use crate::render::renderer::WorldRenderer;
use crate::utils::time::Time;
use crate::{camera::camera_controller::CameraController, world::world::World};

// This is cute, but this is not how a system looks like
// It needs to get the active camera as a component?
pub fn camera_controller_device_sytem(world: &mut World, event: &DeviceEvent) {
    let mut camera_controller = world.get_mut::<CameraController>();
    match event {
        DeviceEvent::MouseMotion { delta } => {
            camera_controller.handle_mouse(delta.0 as f32, delta.1 as f32);
        }
        _ => {}
    }
}

// This spawns a camera
// equates the entity to the renderer value
// creates a bind group cache key
// creates a bind group with that cache key
// set's the bind group via the handle
pub fn camera_init_system(world: &mut World) {
    let camera = Camera::default();
    let entity = world.spawn((camera,));
    let mut renderer = world.get_mut::<WorldRenderer>();
    let mut graphics = world.get_mut::<InternalGraphics>();
    let mut render_queue = world.get_mut::<RenderQueue>();
    renderer.default_camera = Some(entity);
    let cache_key = BindGroupCacheKey {
        layout_num: 0,
        entries: vec![(
            0,
            BindGroupResourceType::Buffer {
                buffer: renderer.camera_buffer.clone(),
            },
        )],
    };
    let camera_bind_group_cache_handle = graphics.get_or_create_bind_group(cache_key);
    render_queue.commands.push(RenderCommand::SetBindGroup {
        bind_group_handle: camera_bind_group_cache_handle,
    });
}

// This then updates the camera
// Gets the camera entity from the renderer
// the window for the aspect ratio
// updates the ratio
// updates the view projection matrix
// writes to the buffer
pub fn camera_update_system(world: &mut World) {
    let renderer = world.get::<WorldRenderer>();
    let graphics = world.get_mut::<InternalGraphics>();
    let window = world.get::<Arc<Window>>();
    if let Some(camera_entity) = renderer.default_camera {
        let window_size = window.inner_size();
        let camera = world.get_entity::<(Camera,)>(camera_entity).0;
        // println!("Camera: {:#?}", TypeId::of::<Camera>());
        camera.aspect = (window_size.width as f32) / window_size.height as f32;
        let view_proj = camera.build_projection_matrix();
        graphics.queue.write_buffer(
            &renderer.camera_buffer,
            0,
            bytemuck::cast_slice(&[view_proj]),
        );
    }
}

pub fn camera_controller_update_system(world: &mut World) {
    //mt anem  is osose and o=im the best in the world i wrote
    //this code
    //

    let renderer = world.get::<WorldRenderer>();
    let camera_entity = renderer.default_camera.unwrap();
    let camera = world.get_entity::<(Camera,)>(camera_entity).0;
    let camera_controller = world.get::<CameraController>();
    let time = world.get::<Time>();
    camera_controller.camera_update(camera, time.time_delta);
}

pub fn camera_window_event(
    world: &mut World,
    window_event: &WindowEvent,
    event_loop: &ActiveEventLoop,
) {
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
                handle_key_controller(
                    &mut world.get_mut::<CameraController>(),
                    *code,
                    key_state.is_pressed(),
                );
            }
        }
        _ => {}
    }
}

// Taking a playbook from cFlake
//
// i can put all these systems into one system called camera system
//
// pub fn camera_system(system: &System) {
// system.init(camera...)
// system.event()
// }
//
//
// so that I can just put them all together
