use bytemuck::{Pod, Zeroable};
use wgpu::{
    Buffer, BufferAddress, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::math::{Vec3, vec3_add, vec3_cross_product, vec3_normalize};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coords: [f32; 2],
    pub normal: Vec3,
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
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 2]>() as BufferAddress,
                    shader_location: 2,
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
    pub index_num_to_draw: u32,
}

impl Mesh {
    pub fn new(device: &Device, vertices: &mut [Vertex], indices: &[u32]) -> Self {
        Self::calculate_normals(vertices, indices);
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
            index_num_to_draw: indices.len() as u32,
            index_buffer: Some(index_buffer),
            vertex_buffer,
        }
    }

    // AI generated this cause I was too lazy to create the normal creation logic, especially
    // because I want to add model loading and most .obj files already have normals
    pub fn cube(device: &Device) -> Self {
        let vertices: &mut [Vertex] = &mut [
            // Front face (z = 0.5) — normal: [0, 0, 1]
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Right face (x = 0.5) — normal: [1, 0, 0]
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            // Left face (x = -0.5) — normal: [-1, 0, 0]
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            // Top face (y = 0.5) — normal: [0, 1, 0]
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];

        let indices: &[u32] = &[
            0, 1, 2, 0, 2, 3, // Front
            4, 5, 6, 4, 6, 7, // Back
            8, 9, 10, 8, 10, 11, // Right
            12, 13, 14, 12, 14, 15, // Left
            16, 17, 18, 16, 18, 19, // Top
            20, 21, 22, 20, 22, 23, // Bottom
        ];

        Self::new(device, vertices, indices)
    }

    pub fn prism(device: &Device) -> Self {
        let vertices: &mut [Vertex] = &mut [
            // Front face (z = 0.5) — normal: [0, 0, 1]
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            // Left face (A-B edge) — normal: perpendicular to edge, pointing outward
            // Normal for left face: [-0.894, 0.447, 0] (normalized)
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            // Right face (A-C edge) — normal: [0.894, 0.447, 0]
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.894, 0.447, 0.0],
            },
        ];

        let indices: &[u32] = &[
            0, 1, 2, // Front
            3, 4, 5, // Back
            6, 7, 8, 6, 8, 9, // Bottom
            10, 11, 12, 10, 12, 13, // Left
            14, 15, 16, 14, 16, 17, // Right
        ];

        Self::new(device, vertices, indices)
    }

    pub fn calculate_normals(vertices: &mut [Vertex], indices: &[u32]) {
        let mut normals: Vec<Vec3> = vec![[0.0, 0.0, 0.0]; vertices.len()];

        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize;
            let i1 = indices[i + 1] as usize;
            let i2 = indices[i + 2] as usize;

            let v0 = vertices[i0].position;
            let v1 = vertices[i1].position;
            let v2 = vertices[i2].position;

            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
            let normal = vec3_cross_product(edge1, edge2);

            normals[i0] = vec3_add(normals[i0], normal);
            normals[i1] = vec3_add(normals[i1], normal);
            normals[i2] = vec3_add(normals[i2], normal);
        }

        for (i, normal) in normals.iter_mut().enumerate() {
            vertices[i].normal = vec3_normalize(*normal);
            println!("Normal: {:?}", vertices[i].normal);
        }
    }
}
