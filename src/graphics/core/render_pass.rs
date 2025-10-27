use std::ops::Range;

pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    pub fn builder() -> RenderPassBuilder<'a> {
        RenderPassBuilder::new()
    }

    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.pass.draw(vertices, instances);
    }
}

pub struct RenderPassBuilder<'a> {
    label: Option<&'a str>,
    clear_color: Option<wgpu::Color>,
    depth_stencil: Option<DepthStencilConfig>,
}

struct DepthStencilConfig {
    clear_depth: Option<f32>,
    clear_stencil: Option<u32>,
}

impl<'a> RenderPassBuilder<'a> {
    pub fn new() -> Self {
        Self {
            label: None,
            clear_color: None,
            depth_stencil: None,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn clear_color(mut self, r: f64, g: f64, b: f64, a: f64) -> Self {
        self.clear_color = Some(wgpu::Color { r, g, b, a });
        self
    }

    pub fn clear_depth(mut self, depth: f32) -> Self {
        if let Some(ref mut config) = self.depth_stencil {
            config.clear_depth = Some(depth);
        } else {
            self.depth_stencil = Some(DepthStencilConfig {
                clear_depth: Some(depth),
                clear_stencil: None,
            });
        }
        self
    }

    pub fn clear_stencil(mut self, stencil: u32) -> Self {
        if let Some(ref mut config) = self.depth_stencil {
            config.clear_stencil = Some(stencil);
        } else {
            self.depth_stencil = Some(DepthStencilConfig {
                clear_depth: None,
                clear_stencil: Some(stencil),
            });
        }
        self
    }

    pub fn begin(
        self,
        encoder: &'a mut wgpu::CommandEncoder,
        color_target: &'a wgpu::TextureView,
        depth_stencil_target: Option<&'a wgpu::TextureView>,
    ) -> RenderPass<'a> {
        let encoder = encoder;
        let color_attachment = wgpu::RenderPassColorAttachment {
            view: color_target,
            resolve_target: None,
            ops: wgpu::Operations {
                load: if let Some(color) = self.clear_color {
                    wgpu::LoadOp::Clear(color)
                } else {
                    wgpu::LoadOp::Load
                },
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        };

        let depth_stencil_attachment = depth_stencil_target.map(|view| {
            let config = self.depth_stencil.as_ref();
            wgpu::RenderPassDepthStencilAttachment {
                view,
                depth_ops: Some(wgpu::Operations {
                    load: config
                        .and_then(|c| c.clear_depth.map(wgpu::LoadOp::Clear))
                        .unwrap_or(wgpu::LoadOp::Load),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: Some(wgpu::Operations {
                    load: config
                        .and_then(|c| c.clear_stencil.map(wgpu::LoadOp::Clear))
                        .unwrap_or(wgpu::LoadOp::Load),
                    store: wgpu::StoreOp::Store,
                }),
            }
        });

        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label,
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        RenderPass { pass }
    }
}
