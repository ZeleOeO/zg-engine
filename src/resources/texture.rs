use anyhow::Ok;
use image::{DynamicImage, ImageBuffer, Rgba};
use image::{GenericImageView, ImageReader};
use wgpu::{
    Device, Extent3d, Origin3d, Queue, Sampler, TexelCopyBufferLayout, TexelCopyTextureInfo,
    Texture, TextureAspect, TextureDescriptor, TextureUsages, TextureView, TextureViewDescriptor,
};

#[derive(Debug)]
pub struct CustomTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub texture_size: Extent3d,
    pub image_rba: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl CustomTexture {
    pub fn from_location(
        device: &wgpu::Device,
        queue: &Queue,
        location: &str,
    ) -> anyhow::Result<Self> {
        let img = ImageReader::open(location)?.decode()?;

        Self::from_image(device, queue, &img)
    }

    pub fn from_image(
        device: &Device,
        queue: &Queue,
        image: &DynamicImage,
    ) -> anyhow::Result<Self> {
        let dimensions = image.dimensions();

        let img_rgba = image.clone().into_rgba8();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture"),
            dimension: wgpu::TextureDimension::D2,
            size: texture_size,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &img_rgba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(img_rgba.dimensions().0 * 4),
                rows_per_image: Some(img_rgba.dimensions().1),
            },
            texture_size,
        );

        Ok(Self {
            texture,
            view: texture_view,
            sampler: texture_sampler,
            texture_size,
            image_rba: img_rgba,
        })
    }
}
