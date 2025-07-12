use crate::components::chart::{
    dto::Drawable,
    grid::GridOptions,
    use_canvas::{ChartData, FontOptions, PaddingOptions},
};
use crate::components::prelude::*;
use kenzu::Builder;
use web_sys::CanvasRenderingContext2d;

#[derive(Builder, Clone, Default, Debug)]
pub struct PieOptions {
    #[set(value = false)]
    pub show_percentage: bool,
    #[set(value = 1.0)]
    pub alpha: f64,
    #[set(value = Color::Transparent)]
    pub stroke_color: Color,
    #[set(value = 1.0)]
    pub line_width: f64,
    #[set(value = true)]
    pub is_doughnut: bool,
    #[set(value = 0.5)]
    pub doughnut_ratio: f64,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct PieChartOptions {
    pub padding: PaddingOptions,
    pub pie: PieOptions,
    pub font: FontOptions,
    pub grid: GridOptions,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct PieChart {
    pub options: PieChartOptions,
    pub data: Vec<ChartData>,
}

impl Drawable for PieChart {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let PieChartOptions {
            padding: pad,
            pie: cfg,
            font: font_cfg,
            grid,
        } = &self.options;
        let total: f64 = self.data.iter().map(|d| d.value).sum();
        if total == 0.0 {
            return;
        }
        grid.draw(ctx, w, h);
        let mut start_angle: f64 = 0.0;
        let pad_top = pad.vertical;
        let pad_bottom = pad.vertical;
        let pad_left = pad.horizontal;
        let pad_right = pad.horizontal;

        let usable_w = w - pad_left - pad_right;
        let usable_h = h - pad_top - pad_bottom;

        let cx = pad_left + usable_w / 2.0;
        let cy = pad_top + usable_h / 2.0;
        let radius = usable_w.min(usable_h) / 2.0;

        for point in &self.data {
            let slice_angle: f64 = (point.value / total) * std::f64::consts::TAU;

            // sombra
            let sh = &point.shadow;
            ctx.set_shadow_blur(sh.blur);
            ctx.set_shadow_color(&sh.color.to_hex());
            ctx.set_shadow_offset_x(sh.offset_x);
            ctx.set_shadow_offset_y(sh.offset_y);

            // preenchimento
            ctx.set_global_alpha(cfg.alpha);
            ctx.set_fill_style_str(&point.color.to_hex());
            ctx.begin_path();

            if cfg.is_doughnut {
                let inner_r = radius * cfg.doughnut_ratio.clamp(0.0, 1.0);
                ctx.move_to(
                    cx + inner_r * start_angle.cos(),
                    cy + inner_r * start_angle.sin(),
                );
                ctx.arc(cx, cy, radius, start_angle, start_angle + slice_angle)
                    .unwrap();
                ctx.arc_with_anticlockwise(
                    cx,
                    cy,
                    inner_r,
                    start_angle + slice_angle,
                    start_angle,
                    true,
                )
                .unwrap();
            } else {
                ctx.move_to(cx, cy);
                ctx.arc(cx, cy, radius, start_angle, start_angle + slice_angle)
                    .unwrap();
            }

            ctx.close_path();
            ctx.fill();

            // contorno, se houver
            if cfg.stroke_color != Color::Transparent && cfg.line_width > 0.0 {
                ctx.set_stroke_style_str(&cfg.stroke_color.to_hex());
                ctx.set_line_width(cfg.line_width);
                ctx.stroke();
            }

            // rótulo
            let mid = start_angle + slice_angle / 2.0;
            let label_r = if cfg.is_doughnut {
                radius * (1.0 + cfg.doughnut_ratio.clamp(0.0, 1.0)) / 2.0
            } else {
                radius * 0.6
            };
            let lx = cx + label_r * mid.cos();
            let ly = cy + label_r * mid.sin();

            ctx.set_shadow_blur(0.0);
            ctx.set_fill_style_str(&font_cfg.text_color.to_hex());
            ctx.set_font(&font_cfg.font);
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");

            let label = if cfg.show_percentage {
                format!("{:.0}%", (point.value / total) * 100.0)
            } else if point.value > 999.0 {
                format!("{:.2e}", point.value)
            } else {
                format!("{:.0}", point.value)
            };
            let _ = ctx.fill_text(&label, lx, ly);

            start_angle += slice_angle;
        }
    }
}
