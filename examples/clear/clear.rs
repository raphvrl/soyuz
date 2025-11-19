use soyuz_app::prelude::*;

use std::f64::consts::TAU;

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

        let t = self.time as f64 * TAU;

        let r = t.sin() * 0.5 + 0.5;
        let g = (t + 2.0).sin() * 0.5 + 0.5;
        let b = (t + 4.0).sin() * 0.5 + 0.5;

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
