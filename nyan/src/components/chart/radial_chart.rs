use crate::components::prelude::*;
use kenzu::Builder;
use web_sys::CanvasRenderingContext2d;

use crate::components::chart::{
    dto::Drawable,
    grid::GridOptions,
    use_canvas::{ChartData, FontOptions, PaddingOptions},
};

#[derive(Builder, Clone, Default, Debug)]
pub struct RadialOptions {
    #[set(value = 1.0)]
    pub alpha: f64,
    #[set(value = Color::Transparent)]
    pub stroke_color: Color,
    #[set(value = false)]
    pub show_percentage: bool,
    #[set(value = true)]
    pub fill_area: bool,
    #[set(value = 15.0)]
    pub label_offset: f64,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct RadialChartOptions {
    pub padding: PaddingOptions,
    pub radial: RadialOptions,
    pub font: FontOptions,
    pub labels: Vec<String>,
    pub grid: GridOptions,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct RadialChart {
    pub options: RadialChartOptions,
    pub data: Vec<ChartData>,
}

impl Drawable for RadialChart {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let opts = &self.options;
        let pad = &opts.padding;
        let center_x = w / 2.0;
        let center_y = h / 2.0;
        let radius = f64::min(w - pad.horizontal * 2.0, h - pad.vertical * 2.0) / 2.0;

        let dimensions = opts.labels.len();
        if dimensions == 0 || self.data.is_empty() || self.data.len() != dimensions {
            return;
        }

        opts.grid.draw(ctx, w, h);
        let angle_step = std::f64::consts::TAU / dimensions as f64;
        let steps = 5;

        // Desenha círculos concêntricos
        for step in 1..=steps {
            let r = radius * step as f64 / steps as f64;
            ctx.begin_path();
            for i in 0..=dimensions {
                let angle = i as f64 * angle_step;
                let x = center_x + angle.cos() * r;
                let y = center_y + angle.sin() * r;
                if i == 0 {
                    ctx.move_to(x, y);
                } else {
                    ctx.line_to(x, y);
                }
            }
            ctx.set_stroke_style_str(&opts.grid.color.to_hex());
            ctx.stroke();
        }

        // Desenha eixos e labels
        for i in 0..dimensions {
            let angle = i as f64 * angle_step;
            let x = center_x + angle.cos() * radius;
            let y = center_y + angle.sin() * radius;

            ctx.begin_path();
            ctx.move_to(center_x, center_y);
            ctx.line_to(x, y);
            ctx.set_stroke_style_str(&opts.grid.color.to_hex());
            ctx.stroke();

            // Desenha labels com fundo para melhor legibilidade
            if let Some(label) = opts.labels.get(i) {
                let label_distance = radius + opts.radial.label_offset;
                let lx = center_x + angle.cos() * label_distance;
                let ly = center_y + angle.sin() * label_distance;

                // Fundo do texto
                ctx.set_fill_style_str("rgba(0,0,0,0.0)");
                let text_width = ctx.measure_text(label).unwrap().width();
                ctx.fill_rect(
                    lx - text_width / 2.0 - 5.0,
                    ly - 10.0,
                    text_width + 10.0,
                    20.0,
                );

                // Texto
                ctx.set_font(&opts.font.font);
                ctx.set_fill_style_str(&opts.font.text_color.to_hex());
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(label, lx, ly);
            }
        }

        // Normalização
        let max = self.data.iter().map(|d| d.value).fold(f64::NAN, f64::max);
        if max <= 0.0 {
            return;
        }

        // Desenha segmentos
        for (i, data) in self.data.iter().enumerate() {
            let pct = data.value / max;
            let angle_start = i as f64 * angle_step;
            let angle_end = (i + 1) as f64 * angle_step;
            let r = pct * radius;

            ctx.begin_path();
            ctx.move_to(center_x, center_y);
            ctx.line_to(
                center_x + angle_start.cos() * r,
                center_y + angle_start.sin() * r,
            );
            ctx.arc(center_x, center_y, r, angle_start, angle_end)
                .unwrap();
            ctx.close_path();

            ctx.set_fill_style_str(&data.color.to_hex());
            ctx.set_stroke_style_str(&opts.radial.stroke_color.to_hex());
            ctx.set_line_width(1.0);

            if opts.radial.fill_area {
                ctx.set_global_alpha(opts.radial.alpha);
                ctx.fill();
                ctx.set_global_alpha(1.0);
            }
            ctx.stroke();
        }
    }
}
