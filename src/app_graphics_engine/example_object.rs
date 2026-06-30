use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferUsages,
    Device, ShaderStages, VertexAttribute, VertexBufferLayout,
    util::{BufferInitDescriptor, DeviceExt},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

#[derive(Debug)]
pub struct ExampleObjectBindGroupData {
    frame: f32,
    frame_buffer: Buffer,
    bind_group: BindGroup,
    bind_group_layout: BindGroupLayout,
}

#[derive(Debug)]
pub struct ExampleObject {
    pub vertex_buffer: Vec<Buffer>,
    pub index_buffer: Option<Vec<Buffer>>,
    pub num_to_draw: u32,
    pub vertex_buffer_layout: Vec<VertexBufferLayout<'static>>,
    pub instances: u32,
    pub frame: f32,

    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

fn create_bind_group(device: &Device) -> ExampleObjectBindGroupData {
    let frame = 0.0;
    let frame_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        contents: bytemuck::cast_slice(&[frame]),
    });

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX,
            count: None,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }],
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: frame_buffer.as_entire_binding(),
        }],
    });

    ExampleObjectBindGroupData {
        frame,
        frame_buffer,
        bind_group,
        bind_group_layout,
    }
}

impl ExampleObject {
    pub fn create_triangle(device: &wgpu::Device) -> Self {
        let mut vertex_data = Vec::new();
        vertex_data.push(Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        }); // Top
        vertex_data.push(Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        }); // Bottom Left
        vertex_data.push(Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        }); // Bottom Right

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer Simple Initialization"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let bind_group_data = create_bind_group(device);

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);

        let mut layouts = Vec::new();
        layouts.push(layout);

        Self {
            vertex_buffer_layout: layouts,
            index_buffer: None,
            num_to_draw: 3,
            instances: 1,
            vertex_buffer: vertex_buffers,

            frame: bind_group_data.frame,
            uniform_buffer: bind_group_data.frame_buffer,
            bind_group_layout: bind_group_data.bind_group_layout,
            bind_group: bind_group_data.bind_group,
        }
    }
    pub fn create_pentagon(device: &Device) -> Self {
        let vertices: &[Vertex] = &[
            Vertex {
                position: [-0.0868241, 0.49240386, 0.0],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [-0.49513406, 0.06958647, 0.0],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [-0.21918549, -0.44939706, 0.0],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [0.35966998, -0.3473291, 0.0],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [0.44147372, 0.2347359, 0.0],
                color: [0.5, 0.0, 0.5],
            },
        ];

        let indices: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            usage: BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(vertices),
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });
        let num_to_draw = indices.len() as u32;

        let layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 1,
                },
            ],
        };

        let bind_group_data = create_bind_group(device);

        let mut v_buffers = Vec::new();
        v_buffers.push(vertex_buffer);
        let mut i_buffers = Vec::new();
        i_buffers.push(index_buffer);
        let mut layouts = Vec::new();
        layouts.push(layout);

        Self {
            frame: bind_group_data.frame,
            vertex_buffer: v_buffers,
            index_buffer: Some(i_buffers),
            num_to_draw,
            vertex_buffer_layout: layouts,
            instances: 1,
            uniform_buffer: bind_group_data.frame_buffer,
            bind_group: bind_group_data.bind_group,
            bind_group_layout: bind_group_data.bind_group_layout,
        }
    }

    pub fn create_spiral(device: &Device, instances: u32) -> Self {
        let vertices = [
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                    shader_location: 1,
                },
            ],
        };
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer Spiral"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        let mut instance_list = Vec::new();
        let points_per_step = 15.0;
        let radius_step_increase = 0.05;
        let angle_per_step = (2.0 * PI) / points_per_step;
        for i in 0..instances {
            let r = i as f32 * radius_step_increase;
            let x = r * (i as f32 * angle_per_step).cos();
            let y = r * (i as f32 * angle_per_step).sin();
            instance_list.push(r);
            instance_list.push(x);
            instance_list.push(y);
        }

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Buffer Spiral"),
            contents: bytemuck::cast_slice(&instance_list),
            usage: BufferUsages::VERTEX,
        });

        let instance_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<f32>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    format: wgpu::VertexFormat::Float32,
                    shader_location: 3,
                },
            ],
        };

        let bind_group_data = create_bind_group(device);

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);
        vertex_buffers.push(instance_buffer);

        let mut layouts = Vec::new();
        layouts.push(vertex_layout);
        layouts.push(instance_layout);

        Self {
            vertex_buffer: vertex_buffers,
            num_to_draw: 3,
            uniform_buffer: bind_group_data.frame_buffer,
            bind_group: bind_group_data.bind_group,
            bind_group_layout: bind_group_data.bind_group_layout,
            index_buffer: None,
            vertex_buffer_layout: layouts,
            frame: bind_group_data.frame,
            instances,
        }
    }
    pub fn create_indexed_example(device: &wgpu::Device) -> Self {
        let mut vertex_data = Vec::new();

        let points = [
            [-1.0, 1.0, 0.0], // 0
            [0.0, 1.0, 0.0],  // 1
            [-0.5, 0.5, 0.0], // 2
            [-1.0, 0.0, 0.0], // 3
            [0.0, 0.5, 0.0],  // 4
            [-0.5, 0.0, 0.0], // 5
            [0.0, 0.0, 0.0],  // 6
            [0.5, 0.0, 0.0],  // 7
            [0.0, -0.5, 0.0], // 8
            [1.0, 0.0, 0.0],  // 9
            [0.5, -0.5, 0.0], // 10
            [0.0, -1.0, 0.0], // 11
            [1.0, -1.0, 0.0], // 12
        ];

        let color = [
            [0.1, 0.0, 0.0], // 0
            [0.1, 0.0, 0.0], // 1
            [1.0, 0.0, 0.0], // 2
            [0.1, 0.0, 0.0], // 3
            [1.0, 0.0, 0.0], // 4
            [1.0, 0.0, 0.0], // 5
            [1.0, 0.0, 0.0], // 6
            [1.0, 0.0, 0.0], // 7
            [1.0, 0.0, 0.0], // 8
            [0.1, 0.0, 0.0], // 9
            [1.0, 0.0, 0.0], // 10
            [0.1, 0.0, 0.0], // 11
            [0.1, 0.0, 0.0], // 12
        ];

        let indices: [[u32; 3]; 18] = [
            [0, 3, 2],    // 0
            [0, 2, 1],    // 1
            [2, 3, 5],    // 2
            [2, 5, 4],    // 3
            [1, 2, 4],    // 4
            [5, 3, 8],    // 5
            [4, 5, 6],    // 6
            [1, 4, 7],    // 7
            [8, 3, 11],   // 8
            [6, 5, 8],    // 9
            [4, 6, 7],    // 10
            [1, 7, 9],    // 11
            [6, 8, 7],    // 12
            [8, 11, 10],  // 13
            [7, 8, 10],   // 14
            [7, 10, 9],   // 15
            [10, 11, 12], // 16
            [9, 10, 12],  // 17
        ];

        for i in 0..points.len() {
            vertex_data.push(Vertex {
                position: points[i],
                color: color[i],
            });
        }

        let mut index_vec = Vec::new();
        for i in 0..indices.len() {
            for k in 0..3 {
                index_vec.push(indices[i][k]);
            }
        }

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Indexed Example Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mut vertex_buffers = Vec::new();
        vertex_buffers.push(vertex_buffer);

        let mut layouts = Vec::new();
        layouts.push(vertex_layout);

        let mut i_buffers = Vec::new();
        i_buffers.push(index_buffer);

        let bind_group_data = create_bind_group(device);

        Self {
            instances: 1,
            num_to_draw: index_vec.len() as u32,
            index_buffer: Some(i_buffers),
            vertex_buffer: vertex_buffers,
            vertex_buffer_layout: layouts,
            bind_group: bind_group_data.bind_group,
            uniform_buffer: bind_group_data.frame_buffer,
            bind_group_layout: bind_group_data.bind_group_layout,
            frame: bind_group_data.frame,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.frame += 1.0;
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[self.frame]));
    }
}
