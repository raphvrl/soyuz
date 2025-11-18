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

        ctx.render(|ctx, view, encoder| {
            let _render_pass = ctx
                .render_pass(encoder, view)
                .clear_rgb(r, g, b)
                .label("Color Cycle Pass")
                .begin();
        });
    }
}

fn main() {
    soyuz_app::run::<ColorCycleApp>("Clear Example");
}
