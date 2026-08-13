use std::{sync::Arc, time::Instant};

use wgpu::{CurrentSurfaceTexture, TextureView, TextureViewDescriptor};
use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent::{self, MouseMotion},
        KeyEvent, WindowEvent,
    },
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::{CursorGrabMode, Window},
};

use crate::{
    camera::{camera::*, camera_controller::CameraController},
    core::gpu::GPU,
    layouts::*,
    math::Vec3,
    passes::render,
    pipeline::{light_pipeline, opaque_pipeline},
    resources::{material::Material, mesh::Mesh, texture::CustomTexture},
    scene::{Object, Scene},
};

use std::rc::Rc;

pub mod camera;
pub mod core;
pub mod key_input;
pub mod layouts;
pub mod math;
pub mod passes;
pub mod pipeline;
pub mod resources;
pub mod scene;

pub struct AppHandler {
    app: Option<App>,
}

pub struct App {
    scene: Scene,
    gpu: GPU,
    window: Arc<Window>,
    camera: Camera,
    camera_controller: CameraController,
    last_frame: Instant,
}

impl App {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let gpu = pollster::block_on(GPU::new(&window)).unwrap();

        let window_size = window.inner_size();
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(winit::window::CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();

        let camera_layout = create_camera_layout(&gpu.device);
        let material_layout = create_material_bg_layout(&gpu.device);
        let item_uniform_layout = create_item_uniform_layout(&gpu.device);
        let light_uniform_layout = create_light_uniform_layout(&gpu.device);

        //Arranged according to bind group
        let layout = [
            Some(&camera_layout),
            Some(&material_layout),
            Some(&item_uniform_layout),
            Some(&light_uniform_layout),
        ];

        let camera = Camera::new((window_size.width as f32) / window_size.height as f32);
        let camera_controller = CameraController::new(2.0, 0.2);
        let camera_uniform = CameraUniform::new(&gpu.device, &camera, &camera_layout);

        let mesh_pipeline = {
            let shader = &gpu
                .device
                .create_shader_module(wgpu::include_wgsl!("./shaders/mesh.wgsl"));
            Rc::new(opaque_pipeline(&gpu.device, &gpu.config, &layout, shader)?)
        };
        let light_pipeline = Rc::new(light_pipeline(&gpu, &layout)?);

        let cube_mesh = Rc::new(Mesh::cube(&gpu.device));
        let prism_mesh = Rc::new(Mesh::prism(&gpu.device));

        // let happy_tree_material = Rc::new(Material::new_texture(
        //     &gpu.device,
        //     &gpu.queue,
        //     &material_layout,
        //     "src/assets/happy-tree.png",
        // )?);
        let brick_material = Rc::new(Material::new_texture(
            &gpu.device,
            &gpu.queue,
            &material_layout,
            "src/assets/brick.jpeg",
        )?);

        let draw_items = vec![
            Object::new_with_texture(
                &gpu,
                &item_uniform_layout,
                &mesh_pipeline,
                &brick_material,
                &prism_mesh,
            )
            .translate(&([1.0, 3.0, 1.0] as Vec3), &gpu),
            Object::new_with_color(
                &gpu,
                &item_uniform_layout,
                &mesh_pipeline,
                Material::new_color(&gpu.device, &material_layout, [1.0, 1.0, 1.0], 1.0)?,
                &prism_mesh,
            )
            .translate(&([1.0, 1.0, 2.0] as Vec3), &gpu)
            .is_light(&light_pipeline),
            Object::new_with_color(
                &gpu,
                &item_uniform_layout,
                &mesh_pipeline,
                Material::new_color(&gpu.device, &material_layout, [1.0, 0.0, 1.0], 0.0)?,
                &cube_mesh,
            )
            .translate(&([2.0, 2.0, 2.0] as Vec3), &gpu),
        ];
        let scene = Scene::new(
            camera_uniform,
            CustomTexture::create_depth_texture(&gpu.device, &gpu.config),
            draw_items,
            &gpu,
            &light_uniform_layout,
        );

        Ok(Self {
            scene,
            gpu: gpu,
            camera,
            window: window,
            camera_controller,
            last_frame: Instant::now(),
        })
    }

    pub fn render(&self, view: &TextureView) {
        self.scene.render_light(&self.gpu);
        render(&self.gpu.device, &self.gpu.queue, view, &self.scene);
    }

    pub fn update(&mut self, dt: f32) {
        let dt_min = dt.min(0.1);
        self.scene
            .camera_uniform
            .update_view_proj(&self.camera, &self.gpu.queue);

        self.camera_controller
            .camera_update(&mut self.camera, dt_min);
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );
        let app = pollster::block_on(App::new(window.clone()));
        self.app = Some(app.unwrap());
        window.request_redraw();
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let Some(app) = &mut self.app else {
            return;
        };

        match event {
            MouseMotion { delta } => {
                app.camera_controller
                    .handle_mouse(delta.0 as f32, delta.1 as f32);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        let Some(app) = &mut self.app else {
            return;
        };
        let dt = now.duration_since(app.last_frame).as_secs_f32();
        app.last_frame = Instant::now();
        app.update(dt);
        app.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(app) = &mut self.app else {
            return;
        };
        match event {
            WindowEvent::Resized(size) => {
                let Some(app) = &mut self.app else {
                    return;
                };
                app.camera.aspect = size.width as f32 / size.height as f32;
                app.scene.depth_texture =
                    CustomTexture::create_depth_texture(&app.gpu.device, &app.gpu.config);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                app.window.request_redraw();

                let frame = match app.gpu.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture)
                    | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                    CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                    CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                        app.gpu.surface.configure(&app.gpu.device, &app.gpu.config);
                        return;
                    }
                    CurrentSurfaceTexture::Validation => return,
                };

                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                app.render(&view);

                frame.present();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => key_input::handle_key(
                code,
                key_state.is_pressed(),
                event_loop,
                &mut app.camera_controller,
            ),
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    use std::env;

    println!("Current directory: {:?}", env::current_dir());
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = AppHandler { app: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}
