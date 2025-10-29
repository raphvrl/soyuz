use super::error::{RenderError, Result};
use super::gpu::GpuContext;
use std::path::Path;

use std::sync::Arc;

pub struct Shader {
    module: wgpu::ShaderModule,
    source: String,
}

impl Shader {
    pub fn from_wgsl(gpu: &GpuContext, source: &str, label: Option<&str>) -> Arc<Self> {
        let device = gpu.device();

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Arc::new(Self {
            module,
            source: source.to_string(),
        })
    }

    pub fn from_file(gpu: &GpuContext, path: impl AsRef<Path>) -> Result<Arc<Self>> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path).map_err(|e| {
            RenderError::ShaderCompilation(format!(
                "Failed to read shader file '{}': {}",
                path.display(),
                e
            ))
        })?;

        let label = path.file_name().and_then(|n| n.to_str());
        Ok(Self::from_wgsl(gpu, &source, label))
    }

    pub fn module(&self) -> &wgpu::ShaderModule {
        &self.module
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

pub struct ShaderBuilder {
    source: Option<String>,
    label: Option<String>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            source: None,
            label: None,
        }
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn from_file(mut self, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path).map_err(|e| {
            RenderError::ShaderCompilation(format!(
                "Failed to read shader file '{}': {}",
                path.display(),
                e
            ))
        })?;

        self.source = Some(source);
        if self.label.is_none() {
            self.label = path.file_name().and_then(|n| n.to_str()).map(String::from);
        }

        Ok(self)
    }

    pub fn build(self, gpu: &GpuContext) -> Result<Arc<Shader>> {
        let source = self
            .source
            .ok_or_else(|| RenderError::ShaderCompilation("No source provided".to_string()))?;

        let label = self.label.as_deref();
        Ok(Shader::from_wgsl(gpu, &source, label))
    }
}

impl Default for ShaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}
