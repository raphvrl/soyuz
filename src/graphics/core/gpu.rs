use crate::graphics::core::error::Result;

pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GpuContext {
    pub fn builder() -> GpuContextBuilder {
        GpuContextBuilder::new()
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter_info(&self) -> wgpu::AdapterInfo {
        self.adapter.get_info()
    }

    pub fn supported_limits(&self) -> wgpu::Limits {
        self.adapter.limits()
    }

    pub fn supported_features(&self) -> wgpu::Features {
        self.adapter.features()
    }

    pub fn supports_feature(&self, feature: wgpu::Features) -> bool {
        self.adapter.features().contains(feature)
    }
}

pub struct GpuContextBuilder {
    backends: wgpu::Backends,
    power_preference: wgpu::PowerPreference,
    required_features: wgpu::Features,
    required_limits: wgpu::Limits,
    compatible_surface: Option<wgpu::Surface<'static>>,
    device_label: Option<String>,
}

impl Default for GpuContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuContextBuilder {
    pub fn new() -> Self {
        Self {
            backends: wgpu::Backends::all(),
            power_preference: wgpu::PowerPreference::HighPerformance,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            compatible_surface: None,
            device_label: Some("Soyuz Device".to_string()),
        }
    }

    pub fn backends(mut self, backends: wgpu::Backends) -> Self {
        self.backends = backends;
        self
    }

    pub fn power_preference(mut self, preference: wgpu::PowerPreference) -> Self {
        self.power_preference = preference;
        self
    }

    pub fn features(mut self, features: wgpu::Features) -> Self {
        self.required_features = features;
        self
    }

    pub fn add_features(mut self, features: wgpu::Features) -> Self {
        self.required_features |= features;
        self
    }

    pub fn limits(mut self, limits: wgpu::Limits) -> Self {
        self.required_limits = limits;
        self
    }

    pub fn compatible_surface(mut self, surface: wgpu::Surface<'static>) -> Self {
        self.compatible_surface = Some(surface);
        self
    }

    pub fn device_label(mut self, label: impl Into<String>) -> Self {
        self.device_label = Some(label.into());
        self
    }

    pub async fn build(self) -> Result<GpuContext> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: self.backends,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: self.power_preference,
                compatible_surface: self.compatible_surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await?;

        let info = adapter.get_info();
        println!("Selected GPU: {} ({:?})", info.name, info.backend);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: self.device_label.as_deref(),
                required_features: self.required_features,
                required_limits: self.required_limits,
                experimental_features: Default::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        Ok(GpuContext {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
