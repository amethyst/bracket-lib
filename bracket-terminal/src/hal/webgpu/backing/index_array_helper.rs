use crate::hal::WgpuLink;
use wgpu::util::DeviceExt;

pub struct IndexBuffer{
    pub data : Vec<u16>,
    pub usage : wgpu::BufferUsages,
    pub buffer: Option<wgpu::Buffer>,
}

impl IndexBuffer
{
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            usage: wgpu::BufferUsages::INDEX,
            buffer: None,
        }
    }

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

    pub fn update_buffer(&mut self, wgpu: &WgpuLink) {
        self.build(wgpu);
    }

    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.as_ref().unwrap().slice(..)
    }
}