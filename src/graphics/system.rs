use std::any::TypeId;

use wgpu::CurrentSurfaceTexture;
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::{
    graphics::gpu::InternalGraphics,
    world::{resources::ResourceMut, world::World},
};

pub fn graphics_window_event_system(
    world: &mut World,
    event: &WindowEvent,
    _event_loop: &ActiveEventLoop,
) {
    match event {
        WindowEvent::RedrawRequested => {
            let graphics = world.get::<InternalGraphics>();
            let frame = match graphics.surface.get_current_texture() {
                CurrentSurfaceTexture::Success(texture)
                | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                    graphics
                        .surface
                        .configure(&graphics.device, &graphics.config);
                    return;
                }
                CurrentSurfaceTexture::Validation => return,
            };

            drop(graphics);
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            println!("Breaks here?");
            world.resource_scope(|world, mut gpu: ResourceMut<InternalGraphics>| {
                println!(
                    "Or breaks here? || TypeId: {:#?}",
                    TypeId::of::<InternalGraphics>()
                );
                gpu.execute(world, &view);
                println!("Or breaks here instead?");
            });
            frame.present();
        }
        _ => {}
    }
}
