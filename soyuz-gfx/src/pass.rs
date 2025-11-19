pub struct RenderPassBuilder<'a> {
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
    clear_color: Option<wgpu::Color>,
    label: Option<&'a str>,
    load_op: Option<wgpu::LoadOp<wgpu::Color>>,
    depth_stencil: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
}

impl<'a> RenderPassBuilder<'a> {
    pub fn new(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView) -> Self {
        Self {
            encoder,
            view,
            clear_color: None,
            label: None,
            load_op: None,
            depth_stencil: None,
        }
    }

    pub fn clear(mut self, color: wgpu::Color) -> Self {
        self.clear_color = Some(color);
        self
    }

    pub fn clear_rgb(mut self, r: f64, g: f64, b: f64) -> Self {
        self.clear_color = Some(wgpu::Color { r, g, b, a: 1.0 });
        self
    }

    pub fn load(mut self) -> Self {
        self.load_op = Some(wgpu::LoadOp::Load);
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn depth_stencil(mut self, attachment: wgpu::RenderPassDepthStencilAttachment<'a>) -> Self {
        self.depth_stencil = Some(attachment);
        self
    }

    pub fn begin(self) -> wgpu::RenderPass<'a> {
        let load_op = self
            .load_op
            .unwrap_or_else(|| wgpu::LoadOp::Clear(self.clear_color.unwrap_or(wgpu::Color::BLACK)));

        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self.label,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: load_op,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: self.depth_stencil,
            timestamp_writes: None,
            occlusion_query_set: None,
        })
    }
}
