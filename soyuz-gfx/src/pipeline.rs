use super::shader::Shader;

pub struct RenderPipelineBuilder<'a> {
    device: &'a wgpu::Device,

    shader: Option<&'a wgpu::ShaderModule>,
    vertex_shader: Option<(&'a wgpu::ShaderModule, &'a str)>,
    fragment_shader: Option<(&'a wgpu::ShaderModule, &'a str)>,
    vertex_entry: Option<&'a str>,
    fragment_entry: Option<&'a str>,

    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,

    color_targets: Vec<Option<wgpu::ColorTargetState>>,
    color_format: wgpu::TextureFormat,
    default_blend: Option<wgpu::BlendState>,
    default_write_mask: wgpu::ColorWrites,

    pipeline_layout: Option<&'a wgpu::PipelineLayout>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,

    label: Option<&'a str>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(device: &'a wgpu::Device, color_format: wgpu::TextureFormat) -> Self {
        Self {
            device,
            shader: None,
            vertex_shader: None,
            fragment_shader: None,
            vertex_entry: Some("vs_main"),
            fragment_entry: Some("fs_main"),
            vertex_buffers: Vec::new(),
            color_targets: Vec::new(),
            color_format,
            default_blend: Some(wgpu::BlendState::REPLACE),
            default_write_mask: wgpu::ColorWrites::ALL,
            pipeline_layout: None,
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            label: None,
        }
    }

    pub fn shader(mut self, shader: &'a wgpu::ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn vertex_shader(mut self, module: &'a wgpu::ShaderModule, entry: &'a str) -> Self {
        self.vertex_shader = Some((module, entry));
        self
    }

    pub fn fragment_shader(mut self, module: &'a wgpu::ShaderModule, entry: &'a str) -> Self {
        self.fragment_shader = Some((module, entry));
        self
    }

    pub fn vertex_shader_from(mut self, shader: &'a Shader, entry: &'a str) -> Self {
        self.vertex_shader = Some((shader.module(), entry));
        self
    }

    pub fn fragment_shader_from(mut self, shader: &'a Shader, entry: &'a str) -> Self {
        self.fragment_shader = Some((shader.module(), entry));
        self
    }

    pub fn vertex_entry(mut self, entry: &'a str) -> Self {
        self.vertex_entry = Some(entry);
        self
    }

    pub fn fragment_entry(mut self, entry: &'a str) -> Self {
        self.fragment_entry = Some(entry);
        self
    }

    pub fn vertex_buffer(mut self, layout: wgpu::VertexBufferLayout<'a>) -> Self {
        self.vertex_buffers.push(layout);
        self
    }

    pub fn blend_state(mut self, blend: wgpu::BlendState) -> Self {
        self.default_blend = Some(blend);
        self
    }

    pub fn color_write_mask(mut self, mask: wgpu::ColorWrites) -> Self {
        self.default_write_mask = mask;
        self
    }

    pub fn color_targets(mut self, targets: Vec<Option<wgpu::ColorTargetState>>) -> Self {
        self.color_targets = targets;
        self
    }

    pub fn pipeline_layout(mut self, layout: &'a wgpu::PipelineLayout) -> Self {
        self.pipeline_layout = Some(layout);
        self
    }

    pub fn depth_stencil(mut self, depth_stencil: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn primitive(mut self, primitive: wgpu::PrimitiveState) -> Self {
        self.primitive = primitive;
        self
    }

    pub fn multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    pub fn no_fragment_shader(mut self) -> Self {
        self.fragment_shader = None;
        self.fragment_entry = None;
        self.shader = None;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn build(self) -> wgpu::RenderPipeline {
        let (vertex_module, vertex_entry) = if let Some((module, entry)) = self.vertex_shader {
            (module, Some(entry))
        } else if let Some(shader) = self.shader {
            (shader, self.vertex_entry)
        } else {
            panic!("Vertex shader must be provided. Use vertex_shader() or shader() method.");
        };

        let color_targets = if self.color_targets.is_empty() {
            vec![Some(wgpu::ColorTargetState {
                format: self.color_format,
                blend: self.default_blend,
                write_mask: self.default_write_mask,
            })]
        } else {
            self.color_targets.clone()
        };

        let fragment_state = if let Some((module, entry)) = self.fragment_shader {
            Some(FragmentStateHelper {
                module,
                entry_point: Some(entry),
                targets: color_targets,
            })
        } else if let Some(shader) = self.shader {
            self.fragment_entry.map(|entry| FragmentStateHelper {
                module: shader,
                entry_point: Some(entry),
                targets: color_targets,
            })
        } else {
            None
        };

        self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label,
                layout: self.pipeline_layout,
                vertex: wgpu::VertexState {
                    module: vertex_module,
                    entry_point: vertex_entry,
                    buffers: &self.vertex_buffers,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: fragment_state.as_ref().map(|fs| wgpu::FragmentState {
                    module: fs.module,
                    entry_point: fs.entry_point,
                    targets: &fs.targets,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                multiview: None,
                cache: None,
            })
    }
}

struct FragmentStateHelper<'a> {
    module: &'a wgpu::ShaderModule,
    entry_point: Option<&'a str>,
    targets: Vec<Option<wgpu::ColorTargetState>>,
}
