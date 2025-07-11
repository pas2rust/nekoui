use crate::components::chart::{
    dto::Drawable,
    grid::GridOptions,
    use_canvas::{ChartData, FontOptions, PaddingOptions},
};
use kenzu::Builder;
use tailwind::components::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[derive(Builder, Clone, Default, Debug)]
pub struct LineOptions {
    #[set(value = 2.0)]
    pub width: f64,
    #[set(value = Color::White)]
    pub color: Color,
    #[set(value = 3.0)]
    pub point_radius: f64,
    #[set(value = Color::White)]
    pub point_color: Color,
}

#[derive(Clone, Default, Debug)]
pub struct LineChartOptions {
    pub padding: PaddingOptions,
    pub line: LineOptions,
    pub font: FontOptions,
    pub grid: GridOptions,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct LineChart {
    pub options: LineChartOptions,
    pub series: Vec<Vec<ChartData>>,
}

impl Drawable for LineChart {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let opts = &self.options;
        let pad = &opts.padding;
        let line_cfg = &opts.line;
        opts.grid.draw(ctx, w, h);
        // calcula valor máximo
        let max_value = self
            .series
            .iter()
            .flat_map(|series| series.iter())
            .map(|d| d.value)
            .fold(f64::NAN, f64::max);
        if max_value.is_nan() || max_value == 0.0 {
            return;
        }
        // desenha cada série
        for series in &self.series {
            if series.len() < 2 {
                continue;
            }
            // calcula espaçamento entre pontos
            let spacing = (w - 2.0 * pad.horizontal) / (series.len() - 1) as f64;
            // gera coordenadas
            let points: Vec<(f64, f64)> = series
                .iter()
                .enumerate()
                .map(|(i, point)| {
                    let x = pad.horizontal + i as f64 * spacing;
                    let y = h - pad.global - (point.value / max_value) * (h - 2.0 * pad.global);
                    (x, y)
                })
                .collect();
            // desenha linhas
            ctx.set_line_width(line_cfg.width);
            ctx.set_shadow_blur(0.0);
            for i in 0..points.len() - 1 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[i + 1];
                ctx.begin_path();
                ctx.set_stroke_style_str(&series[i].color.to_hex());
                ctx.move_to(x1, y1);
                ctx.line_to(x2, y2);
                ctx.stroke();
            }
            // desenha pontos e rótulos
            for (i, point) in series.iter().enumerate() {
                let (x, y) = points[i];
                // ponto
                ctx.set_fill_style_str(&point.color.to_hex());
                ctx.begin_path();
                ctx.arc(x, y, line_cfg.point_radius, 0.0, std::f64::consts::TAU)
                    .unwrap();
                ctx.fill();
                // rótulo
                let font_cfg = &point.font;
                ctx.set_font(&font_cfg.font);
                ctx.set_fill_style_str(&font_cfg.text_color.to_hex());
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let label = if point.value > 999.0 {
                    format!("{:.2e}", point.value)
                } else {
                    format!("{:.0}", point.value)
                };
                let _ = ctx.fill_text(&label, x, y - 5.0);
            }
        }
    }
}
