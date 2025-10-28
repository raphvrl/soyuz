use super::bind_group::BindGroup;
use super::error::{RenderError, Result};
use super::shader::Shader;
use std::sync::Arc;

pub use wgpu::{
    BlendState, BufferAddress, ColorWrites, CompareFunction, Face, FrontFace, PolygonMode,
    PrimitiveTopology, TextureFormat, VertexAttribute, VertexFormat, VertexStepMode,
};

pub struct RenderPipeline {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::PipelineLayout,
}

impl RenderPipeline {
    pub fn builder() -> PipelineBuilder {
        PipelineBuilder::new()
    }

    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    pub fn layout(&self) -> &wgpu::PipelineLayout {
        &self.layout
    }
}

pub struct PipelineBuilder {
    label: Option<String>,
    vertex_shader: Option<(Arc<Shader>, String)>,
    fragment_shader: Option<(Arc<Shader>, String)>,
    vertex_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    push_constant_ranges: Vec<wgpu::PushConstantRange>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    color_targets: Vec<Option<wgpu::ColorTargetState>>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            label: None,
            vertex_shader: None,
            fragment_shader: None,
            vertex_layouts: Vec::new(),
            bind_group_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            color_targets: vec![Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn vertex_shader(mut self, shader: Arc<Shader>, entry_point: &str) -> Self {
        self.vertex_shader = Some((shader, entry_point.to_string()));
        self
    }

    pub fn fragment_shader(mut self, shader: Arc<Shader>, entry_point: &str) -> Self {
        self.fragment_shader = Some((shader, entry_point.to_string()));
        self
    }

    pub fn vertex_layout(mut self, layout: wgpu::VertexBufferLayout<'static>) -> Self {
        self.vertex_layouts.push(layout);
        self
    }

    pub fn vertex_layouts(mut self, layouts: &[wgpu::VertexBufferLayout<'static>]) -> Self {
        self.vertex_layouts.extend_from_slice(layouts);
        self
    }

    pub fn bind_group_layout(mut self, layout: &BindGroup) -> Self {
        self.bind_group_layouts.push(layout.layout().clone());
        self
    }

    pub fn raw_bind_group_layout(mut self, layout: &wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout.clone());
        self
    }

    pub fn push_constant_range(mut self, range: wgpu::PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    pub fn topology(mut self, topology: wgpu::PrimitiveTopology) -> Self {
        self.primitive.topology = topology;
        self
    }

    pub fn polygon_mode(mut self, mode: wgpu::PolygonMode) -> Self {
        self.primitive.polygon_mode = mode;
        self
    }

    pub fn front_face(mut self, front_face: wgpu::FrontFace) -> Self {
        self.primitive.front_face = front_face;
        self
    }

    pub fn cull_mode(mut self, cull_mode: Option<wgpu::Face>) -> Self {
        self.primitive.cull_mode = cull_mode;
        self
    }

    pub fn depth_stencil(mut self, format: wgpu::TextureFormat) -> Self {
        self.depth_stencil = Some(wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        self
    }

    pub fn depth_stencil_state(mut self, state: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(state);
        self
    }

    pub fn color_target(mut self, format: wgpu::TextureFormat) -> Self {
        self.color_targets = vec![Some(wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        self
    }

    pub fn color_target_blend(
        mut self,
        format: wgpu::TextureFormat,
        blend: wgpu::BlendState,
    ) -> Self {
        self.color_targets = vec![Some(wgpu::ColorTargetState {
            format,
            blend: Some(blend),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        self
    }

    pub fn alpha_blending(mut self, format: wgpu::TextureFormat) -> Self {
        self.color_targets = vec![Some(wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        })];
        self
    }

    pub fn multisample_count(mut self, count: u32) -> Self {
        self.multisample.count = count;
        self
    }

    pub fn build(self, device: &wgpu::Device) -> Result<RenderPipeline> {
        let vertex_shader = self.vertex_shader.ok_or(RenderError::MissingVertexShader)?;
        let fragment_shader = self.fragment_shader;

        let bind_group_layout_refs: Vec<&wgpu::BindGroupLayout> =
            self.bind_group_layouts.iter().collect();

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: self
                .label
                .as_ref()
                .map(|s| format!("{} Layout", s))
                .as_deref(),
            bind_group_layouts: &bind_group_layout_refs,
            push_constant_ranges: &self.push_constant_ranges,
        });

        let vertex_entry = &vertex_shader.1;
        let fragment_entry = fragment_shader
            .as_ref()
            .map(|(_, entry)| entry.as_str())
            .unwrap_or("fs_main");

        let vertex_state = wgpu::VertexState {
            module: vertex_shader.0.module(),
            entry_point: Some(vertex_entry),
            buffers: &self.vertex_layouts,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        };

        let fragment_state = match &fragment_shader {
            Some((shader, _)) => Some(wgpu::FragmentState {
                module: shader.module(),
                entry_point: Some(fragment_entry),
                targets: &self.color_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            None => None,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label.as_deref(),
            layout: Some(&layout),
            vertex: vertex_state,
            primitive: self.primitive,
            depth_stencil: self.depth_stencil,
            multisample: self.multisample,
            fragment: fragment_state,
            multiview: None,
            cache: None,
        });

        Ok(RenderPipeline { pipeline, layout })
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn vertex_layout<'a>(
    attributes: &'a [wgpu::VertexAttribute],
    stride: wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode,
) -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode,
        attributes,
    }
}
