use bytemuck::{Pod, Zeroable};
use wgpu::{
    Buffer, BufferAddress, BufferSlice, BufferUsages, RenderPass, VertexAttribute,
    VertexBufferLayout, VertexFormat,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    core::gpu::InternalGraphics,
    math::{Vec3, vec3_add, vec3_cross_product, vec3_normalize},
};

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

pub struct MeshManager {
    vertex_buffers: Vec<Buffer>,
    index_buffers: Vec<Buffer>,
    meshes: Vec<MeshMetaData>,
}

#[derive(Clone, Copy)]
pub struct MeshHandle(pub u32);

// This is for draw
pub struct MeshMetaData {
    pub vertex_count: u32,
    pub index_count: u32,
    pub byte_size: usize,
}

// Right now, only vertices with indices can be drawn
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl MeshManager {
    pub fn new() -> MeshManager {
        MeshManager {
            vertex_buffers: Vec::new(),
            index_buffers: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn add_mesh_data(
        &mut self,
        mesh_data: &mut MeshData,
        gpu: &InternalGraphics,
    ) -> MeshHandle {
        Self::calculate_normals(mesh_data);
        let vertex_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.vertices),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&mesh_data.indices),
            usage: BufferUsages::INDEX,
        });

        self.meshes.push(MeshMetaData {
            vertex_count: mesh_data.vertices.len() as u32,
            index_count: mesh_data.indices.len() as u32,
            byte_size: index_buffer.size() as usize,
        });

        self.vertex_buffers.push(vertex_buffer);
        self.index_buffers.push(index_buffer);

        MeshHandle(self.vertex_buffers.len() as u32 - 1)
    }

    fn calculate_normals(mesh_data: &mut MeshData) {
        let mut normals: Vec<Vec3> = vec![[0.0, 0.0, 0.0]; mesh_data.vertices.len()];

        for i in (0..mesh_data.indices.len()).step_by(3) {
            let i0 = mesh_data.indices[i] as usize;
            let i1 = mesh_data.indices[i + 1] as usize;
            let i2 = mesh_data.indices[i + 2] as usize;

            let v0 = mesh_data.vertices[i0].position;
            let v1 = mesh_data.vertices[i1].position;
            let v2 = mesh_data.vertices[i2].position;

            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
            let normal = vec3_cross_product(edge1, edge2);

            normals[i0] = vec3_add(normals[i0], normal);
            normals[i1] = vec3_add(normals[i1], normal);
            normals[i2] = vec3_add(normals[i2], normal);
        }

        for (i, normal) in normals.iter_mut().enumerate() {
            mesh_data.vertices[i].normal = vec3_normalize(*normal);
        }
    }

    pub fn get_buffers_as_slice(&self, mesh_handle: u32) -> (BufferSlice, BufferSlice) {
        let vertex_slice = self.vertex_buffers[mesh_handle as usize].slice(..);
        let index_slice = self.index_buffers[mesh_handle as usize].slice(..);

        (vertex_slice, index_slice)
        // render_pass.draw_indexed(0..self.meshes[mesh_handle as usize].index_count, 0, 0..1);
    }
}
