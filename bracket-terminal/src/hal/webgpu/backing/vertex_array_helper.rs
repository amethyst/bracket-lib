use wgpu::util::DeviceExt;
use crate::hal::WgpuLink;

pub struct FloatBuffer<T>
where
    T: bytemuck::Pod,
{
    pub data: Vec<T>,
    pub attributes: Vec<wgpu::VertexAttribute>,
    total_size: wgpu::BufferAddress,
    row_len: usize,
    pub buffer: Option<wgpu::Buffer>,
    usage: wgpu::BufferUsages,
}

impl<T> FloatBuffer<T>
where
    T: bytemuck::Pod,
{
    pub fn new(layout: &[usize], capacity: usize, usage: wgpu::BufferUsages) -> Self {
        let mut attributes = Vec::with_capacity(capacity);

        let mut cumulative_len = 0;
        let mut cumulative_size = 0;
        for (i, size) in layout.iter().enumerate() {
            let attribute = wgpu::VertexAttribute {
                offset: cumulative_size,
                shader_location: i as u32,
                format: match size {
                    1 => wgpu::VertexFormat::Float32,
                    2 => wgpu::VertexFormat::Float32x2,
                    3 => wgpu::VertexFormat::Float32x3,
                    4 => wgpu::VertexFormat::Float32x4,
                    _ => {
                        panic!("Vertices must be 1-4 floats");
                    }
                },
            };
            attributes.push(attribute);
            cumulative_size += (std::mem::size_of::<T>() * size) as wgpu::BufferAddress;
            cumulative_len += size;
        }

        Self {
            data: Vec::new(),
            attributes,
            total_size: cumulative_size,
            row_len: cumulative_len,
            buffer: None,
            usage,
        }
    }

    pub fn descriptor(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.total_size,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
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
        (self.data.len() / self.row_len) as u32
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.as_ref().unwrap().slice(..)
    }
}
