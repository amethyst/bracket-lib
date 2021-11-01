//! Helper for generating and maintaining WGPU index buffers.

use crate::hal::WgpuLink;
use wgpu::util::DeviceExt;

/// Provides a wgpu index buffer for indexed draw operations.
pub struct IndexBuffer {
    /// The actual buffer data.
    pub data: Vec<u16>,
    /// Usage list. Generally wgpu::BufferUsages::INDEX
    pub usage: wgpu::BufferUsages,
    /// The actual WGPU buffer to which the data is mapped.
    pub buffer: Option<wgpu::Buffer>,
}

impl IndexBuffer {
    /// Create a new index buffer with a given vector capacity.
    /// Note that capacity is not the same as zeroing it - it just
    /// reserves vector space.
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            usage: wgpu::BufferUsages::INDEX,
            buffer: None,
        }
    }

    /// Calls WGPU's "create_buffer_init" path to copy the index buffer
    /// from local memory to GPU memory.
    pub fn build(&mut self, wgpu: &WgpuLink) {
        if let Some(buf) = &mut self.buffer {
            std::mem::drop(buf);
        }
        self.buffer = Some(
            wgpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&self.data),
                    usage: self.usage,
                }),
        );
    }

    /// Returns the length of the index buffer
    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    /// Maps the index buffer into a slice, suitable for render submission.
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.as_ref().unwrap().slice(..)
    }
}
