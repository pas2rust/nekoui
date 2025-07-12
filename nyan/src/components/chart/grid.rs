use crate::components::chart::dto::Drawable;
use crate::components::prelude::*;
use kenzu::Builder;
use web_sys::CanvasRenderingContext2d;

#[derive(Builder, Clone, Default, Debug)]
pub struct GridOptions {
    #[set(value = true)]
    pub show: bool,
    #[set(value = 20.0)]
    pub size: f64,
    #[set(value = Color::White)]
    pub color: Color,
    #[set(value = 0.5)]
    pub alpha: f64,
    #[set(value = 1.0)]
    pub line_width: f64,
}

impl Drawable for GridOptions {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let opts = &self;

        if !opts.show || opts.size <= 0.0 {
            return;
        }

        ctx.set_stroke_style_str(&opts.color.to_hex());
        ctx.set_line_width(opts.line_width);
        ctx.set_global_alpha(opts.alpha);
        ctx.begin_path();

        let step = opts.size.max(1.0);

        // Linhas horizontais
        for y in (0..=(h as usize)).step_by(step as usize) {
            let y = y as f64;
            ctx.move_to(0.0, y);
            ctx.line_to(w, y);
        }

        // Linhas verticais
        for x in (0..=(w as usize)).step_by(step as usize) {
            let x = x as f64;
            ctx.move_to(x, 0.0);
            ctx.line_to(x, h);
        }

        ctx.stroke();
        ctx.set_global_alpha(1.0); // Reset alpha
    }
}
