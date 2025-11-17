use soyuz_app::prelude::*;

struct ColorCycleApp {
    time: f32,
}

impl App for ColorCycleApp {
    fn init(_ctx: &mut Context) -> Self {
        Self { time: 0.0 }
    }

    fn frame(&mut self, ctx: &mut Context, dt: f32) {
        self.time += dt;
        if self.time > 1.0 {
            self.time = 0.0;
        }

        let r = (self.time as f64 * 6.28).sin() * 0.5 + 0.5;
        let g = ((self.time as f64 * 6.28) + 2.0).sin() * 0.5 + 0.5;
        let b = ((self.time as f64 * 6.28) + 4.0).sin() * 0.5 + 0.5;

        ctx.render(|view, _device, _queue, encoder| {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Color Cycle Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        });
    }
}

fn main() {
    soyuz_app::run::<ColorCycleApp>("Clear Example");
}
