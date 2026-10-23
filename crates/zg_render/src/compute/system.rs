use std::sync::mpsc::channel;

use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor,
    util::{BufferInitDescriptor, DeviceExt},
};
use zg_graphics::{BindGroupCacheKey, BindGroupResourceType, ComputePipelineID, InternalGraphics};
use zg_world::{Res, ResMut, Resource};

use crate::{ComputeItem, compute::compute_queue::ComputeQueue};

#[derive(Debug, Resource, Default)]
pub struct RandomTestStruct {
    test_buffer: Option<Buffer>,
}

// NOTE: Runs  after compute_test_system
// runs before  do some compute stuff
pub fn compute_system(graphics: Res<InternalGraphics>, mut compute_queue: ResMut<ComputeQueue>) {
    let mut encoder = graphics
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });
    {
        let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some("Compute Pass"),
            ..Default::default()
        });

        compute_queue.flush(&mut compute_pass, &graphics);
    }
    {
        compute_queue.copy_to_copy(&mut encoder);
    }

    graphics.queue.submit(Some(encoder.finish()));
}

pub fn compute_setup_test_sytem(
    mut graphics: ResMut<InternalGraphics>,
    mut compute_queue: ResMut<ComputeQueue>,
    mut test_struct: ResMut<RandomTestStruct>,
) {
    // create an input buffer that's an array
    // create an output buffer that i can write to
    // create a buffer, readbuffer that i can read from in theh cpu

    let input_data = (0..10_000u32).collect::<Vec<_>>();

    let input_buffer = graphics.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
        contents: bytemuck::cast_slice(&input_data),
    });

    let output_buffer = graphics.device.create_buffer(&BufferDescriptor {
        label: None,
        size: size_of::<f32>() as u64,
        usage: BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let read_buffer = graphics.device.create_buffer(&BufferDescriptor {
        label: None,
        size: size_of::<f32>() as u64,
        usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    test_struct.test_buffer = Some(read_buffer.clone());

    let cache_key = BindGroupCacheKey {
        layout_num: None,
        entries: vec![
            (
                0,
                BindGroupResourceType::Buffer {
                    buffer: input_buffer,
                },
            ),
            (
                1,
                BindGroupResourceType::Buffer {
                    buffer: output_buffer.clone(),
                },
            ),
        ],
    };
    let bind_group_handle = graphics.get_or_create_bind_group(cache_key);

    compute_queue.add_compute_item(ComputeItem {
        bind_group_handle,
        pipeline_id: ComputePipelineID::COMPUTE,
    });

    compute_queue.add_copy_to_copy_buffers(output_buffer, read_buffer);
}

pub fn do_some_compute_stuff(test_struct: Res<RandomTestStruct>, graphics: Res<InternalGraphics>) {
    let temp_buffer = test_struct.test_buffer.clone().unwrap();
    {
        let (tx, rx) = channel();

        temp_buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            tx.send(result).unwrap()
        });

        graphics
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();

        rx.recv().unwrap().unwrap();

        let output_data = temp_buffer.get_mapped_range(..).unwrap();

        println!("Ladies and gentelemen, we got it: {:#?}", output_data);
    }

    temp_buffer.unmap();
}
