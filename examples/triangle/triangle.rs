use soyuz_app::prelude::*;

struct TriangleApp {
    pipeline: wgpu::RenderPipeline,
}

impl App for TriangleApp {
    fn init(ctx: &mut Context) -> Self {
        let shader = ctx.shader(include_str!("triangle.wgsl"));

        let pipeline = ctx
            .render_pipeline()
            .shader(shader.module())
            .label("Triangle Pipeline")
            .build();

        Self { pipeline }
    }

    fn frame(&mut self, ctx: &mut Context, _dt: f32) {
        ctx.render(|view, _device, _queue, encoder| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Triangle Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
        });
    }
}

fn main() {
    soyuz_app::run::<TriangleApp>("Triangle Example");
}
