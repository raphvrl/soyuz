use thiserror::Error;
use wgpu;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("No suitable GPU adapter found")]
    NoSuitableAdapter,

    #[error("Failed to request adapter: {0}")]
    AdapterRequest(#[from] wgpu::RequestAdapterError),

    #[error("Failed to request device: {0}")]
    DeviceRequest(#[from] wgpu::RequestDeviceError),

    #[error("Failed to create surface: {0}")]
    SurfaceCreationFailed(#[from] wgpu::CreateSurfaceError),

    #[error("Surface error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),

    #[error("Shader compilation failed: {0}")]
    ShaderCompilation(String),

    #[error("Pipeline creation failed: {0}")]
    PipelineCreation(String),

    #[error("Invalid pipeline configuration: {0}")]
    InvalidPipelineConfiguration(String),

    #[error("Missing vertex shader")]
    MissingVertexShader,

    #[error("Missing fragment shader")]
    MissingFragmentShader,
}

pub type Result<T> = std::result::Result<T, RenderError>;
