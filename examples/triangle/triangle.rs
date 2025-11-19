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
        ctx.render(|ctx, view, encoder| {
            let mut render_pass = ctx
                .render_pass(encoder, view)
                .clear_rgb(0.1, 0.1, 0.1)
                .label("Triangle Render Pass")
                .begin();

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
        });
    }
}

fn main() {
    soyuz_app::run::<TriangleApp>("Triangle Example");
}
