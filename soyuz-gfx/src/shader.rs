pub struct Shader {
    module: wgpu::ShaderModule,
}

impl Shader {
    pub fn from_wgsl(device: &wgpu::Device, source: &str) -> Self {
        Self {
            module: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            }),
        }
    }

    pub fn from_wgsl_file(device: &wgpu::Device, path: &str) -> Result<Self, std::io::Error> {
        let source = std::fs::read_to_string(path)?;
        Ok(Self::from_wgsl(device, &source))
    }

    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }
}
