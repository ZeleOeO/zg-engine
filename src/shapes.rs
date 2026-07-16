use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device,
    ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::app_graphics_engine::Vertex;

#[derive(Debug)]
pub struct Shapes {
    pub index_buffer: Option<Buffer>,
    pub vertex_buffer: Buffer,
    pub num_to_draw: u32,
    pub uniform_buffer: Option<Buffer>,
    pub uniform_buffer_bind_group_layout: Option<BindGroupLayout>,
    pub uniform_buffer_bind_group: Option<BindGroup>,
}

impl Shapes {
    pub fn display_shapes(device: &Device) -> Self {
        let vertices: &[Vertex] = &[
            // Front face (z = 0.5)
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
            },
            // Back face (z = -0.5)
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            // Right face (x = 0.5)
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
            },
            // Left face (x = -0.5)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            // Top face (y = 0.5)
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
            },
            // Bottom face (y = -0.5)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices: &[u32] = &[
            // Front
            0, 1, 2, 0, 2, 3, // Back
            4, 5, 6, 4, 6, 7, // Right
            8, 9, 10, 8, 10, 11, // Left
            12, 13, 14, 12, 14, 15, // Top
            16, 17, 18, 16, 18, 19, // Bottom
            20, 21, 22, 20, 22, 23,
        ];

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Shape Uniform Buffer"),
            size: std::mem::size_of::<f32>() as BufferAddress,
            mapped_at_creation: false,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_buffer_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Uniform Buffer Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Buffer Bind Group"),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            layout: &uniform_buffer_bind_group_layout,
        });

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            num_to_draw: vertices.len() as u32,
            index_buffer: Some(index_buffer),
            vertex_buffer,
            uniform_buffer: Some(uniform_buffer),
            uniform_buffer_bind_group_layout: Some(uniform_buffer_bind_group_layout),
            uniform_buffer_bind_group: Some(uniform_buffer_bind_group),
        }
    }
}
