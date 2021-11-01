//! Helper to load a shader from WGSL
use wgpu::ShaderModule;

pub struct Shader(pub ShaderModule);

impl Shader {
    pub fn new(device: &wgpu::Device, source: &str) -> Self {
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Shader(shader)
    }
}
