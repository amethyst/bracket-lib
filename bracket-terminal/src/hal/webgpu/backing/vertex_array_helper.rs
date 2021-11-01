//! Provides a wrapper for wgpu vertex buffers, including attribute mapping.
use crate::hal::WgpuLink;
use wgpu::util::DeviceExt;

/// Maps a vector to a wgpu buffer.
pub struct FloatBuffer<T>
where
    T: bytemuck::Pod,
{
    /// The underlying buffer data
    pub data: Vec<T>,
    /// A map of attribute types represented by this buffer
    pub attributes: Vec<wgpu::VertexAttribute>,
    /// The total size of a vertex buffer row
    total_size: wgpu::BufferAddress,
    /// The actual WGPU buffer, if any.
    pub buffer: Option<wgpu::Buffer>,
    /// Usages, generally set to VERTEX
    usage: wgpu::BufferUsages,
}

impl<T> FloatBuffer<T>
where
    T: bytemuck::Pod,
{
    /// Create a new vertex buffer. Layout specifies the size of each
    /// attribute. E.g. [4,3,2] will use float4, float3, float2 entries.
    /// Capacity is vector capacity.
    /// Usage is usually VERTEX.
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
            buffer: None,
            usage,
        }
    }

    /// Create a vertex buffer descriptor for pipelines.
    pub fn descriptor(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.total_size,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }

    /// If a previous buffer exists, drop it.
    /// Map the backing store to a new vertex array.
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

    /// Maps the vertex buffer to a render-friendly slice.
    pub fn slice(&self) -> wgpu::BufferSlice {
        self.buffer.as_ref().unwrap().slice(..)
    }
}
