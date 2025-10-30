use super::error::Result;

pub struct BindGroup {
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}

impl BindGroup {
    pub fn builder() -> BindGroupBuilder {
        BindGroupBuilder::new()
    }

    pub fn raw(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

pub struct BindGroupBuilder {
    label: Option<String>,
    layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindGroupBuilder {
    pub fn new() -> Self {
        Self {
            label: None,
            layout_entries: Vec::new(),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn uniform(mut self, binding: u32, visibility: wgpu::ShaderStages) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        self
    }

    pub fn storage(mut self, binding: u32, visibility: wgpu::ShaderStages) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        self
    }

    pub fn storage_read_write(mut self, binding: u32, visibility: wgpu::ShaderStages) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        self
    }

    pub fn texture(mut self, binding: u32, visibility: wgpu::ShaderStages) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        });
        self
    }

    pub fn sampler(mut self, binding: u32, visibility: wgpu::ShaderStages) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        });
        self
    }

    pub fn texture_array(
        mut self,
        binding: u32,
        visibility: wgpu::ShaderStages,
        count: u32,
    ) -> Self {
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty: wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            count: std::num::NonZeroU32::new(count),
        });
        self
    }

    pub fn build(self, device: &wgpu::Device, resources: &[BindResource]) -> Result<BindGroup> {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: self
                .label
                .as_ref()
                .map(|s| format!("{} Layout", s))
                .as_deref(),
            entries: &self.layout_entries,
        });

        let entries: Vec<wgpu::BindGroupEntry> = resources
            .iter()
            .map(|resource| match resource {
                BindResource::Sampler(binding, sampler) => wgpu::BindGroupEntry {
                    binding: *binding,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
                BindResource::TextureView(binding, texture_view) => wgpu::BindGroupEntry {
                    binding: *binding,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                BindResource::TextureViewArray(binding, texture_views) => wgpu::BindGroupEntry {
                    binding: *binding,
                    resource: wgpu::BindingResource::TextureViewArray(texture_views),
                },
            })
            .collect();

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label.as_deref(),
            layout: &layout,
            entries: &entries,
        });

        Ok(BindGroup { bind_group, layout })
    }

    pub fn layout_only(self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: self.label.as_deref(),
            entries: &self.layout_entries,
        })
    }
}

impl Default for BindGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub enum BindResource<'a> {
    Sampler(u32, &'a wgpu::Sampler),
    TextureView(u32, &'a wgpu::TextureView),
    TextureViewArray(u32, &'a [&'a wgpu::TextureView]),
}
