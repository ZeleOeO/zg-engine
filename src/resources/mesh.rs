use bytemuck::{Pod, Zeroable};
use wgpu::{
    Buffer, BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    util::{BufferInitDescriptor, DeviceExt},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    shader_location: 0,
                    offset: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub index_buffer: Option<Buffer>,
    pub vertex_buffer: Buffer,
    pub num_to_draw: u32,
}

impl Mesh {
    pub fn new(device: &Device, vertices: &[Vertex], indices: &[u32]) -> Self {
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
        }
    }

    pub fn cube(device: &Device) -> Self {
        // We can do size and position later with math
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

        Self::new(device, vertices, indices)
    }

    pub fn prism(device: &Device) -> Self {
        // Triangular prism (equilateral-ish cross-section extruded along z)
        // Apex A = top, B = bottom-left, C = bottom-right
        let vertices: &[Vertex] = &[
            // Front face (z = 0.5) — triangle A, B, C
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
            },
            // Back face (z = -0.5) — triangle A, C, B (reversed winding)
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.5, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
            },
            // Bottom face (y = -0.5), between B and C
            Vertex {
                position: [-0.5, -0.5, 0.5], // B_front
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5], // B_back
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5], // C_back
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5], // C_front
                tex_coords: [0.0, 0.0],
            },
            // Left face (A-B edge)
            Vertex {
                position: [0.0, 0.5, 0.5], // A_front
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5], // A_back
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5], // B_back
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5], // B_front
                tex_coords: [0.0, 0.0],
            },
            // Right face (A-C edge)
            Vertex {
                position: [0.0, 0.5, 0.5], // A_front
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5], // C_front
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5], // C_back
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5], // A_back
                tex_coords: [0.0, 0.0],
            },
        ];
        let indices: &[u32] = &[
            // Front triangle
            0, 1, 2, // Back triangle
            3, 4, 5, // Bottom
            6, 7, 8, 6, 8, 9, // Left
            10, 11, 12, 10, 12, 13, // Right
            14, 15, 16, 14, 16, 17,
        ];
        Self::new(device, vertices, indices)
    }
}
