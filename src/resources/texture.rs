use anyhow::Ok;
use image::DynamicImage;
use image::{GenericImageView, ImageReader};
use wgpu::wgt::TextureDescriptor;
use wgpu::{
    CompareFunction, Device, Extent3d, Origin3d, Queue, Sampler, SamplerDescriptor,
    SurfaceConfiguration, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect,
    TextureFormat, TextureUsages, TextureView, TextureViewDescriptor,
};

#[derive(Debug)]
pub struct CustomTexture {
    pub view: TextureView,
    pub sampler: Sampler,
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
            view: texture_view,
            sampler: texture_sampler,
        })
    }

    pub fn create_depth_texture(device: &Device, config: &SurfaceConfiguration) -> Self {
        let size = Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            format: wgpu::TextureFormat::Depth32Float,
            mip_level_count: 1,
            size: size,
            dimension: wgpu::TextureDimension::D2,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            view: texture_view,
            sampler: texture_sampler,
        }
    }

    pub fn create_dummy_texture(device: &Device) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Dummy Texture"),
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("Dummy Sampler"),
            ..Default::default()
        });

        Self {
            view: texture_view,
            sampler,
        }
    }
}
