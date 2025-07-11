use kenzu::Builder;
use web_sys::CanvasRenderingContext2d;

use crate::components::{
    _shared::color::Color,
    chart::{
        dto::Drawable,
        grid::GridOptions,
        use_canvas::{ChartData, PaddingOptions},
    },
};

#[derive(Clone, Default, Debug)]
pub enum Direction {
    #[default]
    BottomUp,
    TopDown,
    LeftRight,
    RightLeft,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct BarOptions {
    #[set(value = 0.0)]
    pub border_radius: f64,
    #[set(value = 1.0)]
    pub alpha: f64,
    #[set(value = 1.0)]
    pub line_width: f64,
    #[set(value = Color::Transparent)]
    pub stroke_color: Color,
}

#[derive(Clone, Default, Debug, Builder)]
pub struct BarChartOptions {
    pub direction: Direction,
    pub padding: PaddingOptions,
    pub bar: BarOptions,
    pub grid: GridOptions,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct BarChart {
    pub options: BarChartOptions,
    pub data: Vec<ChartData>,
}

impl Drawable for BarChart {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let opts = &self.options;
        let pad = &opts.padding;
        let bar_cfg = &opts.bar;
        opts.grid.draw(ctx, w, h);
        // calcula valor máximo
        let max = self.data.iter().map(|d| d.value).fold(f64::NAN, f64::max);
        if max.is_nan() || max == 0.0 {
            return;
        }
        // para cada ponto, desenha
        for (i, point) in self.data.iter().enumerate() {
            // calcula posição conforme direção
            let (x, y, width, height, tx, ty) = match opts.direction {
                Direction::BottomUp => {
                    let space = (w - 2.0 * pad.horizontal) / self.data.len() as f64;
                    let thick = space * 0.6;
                    let x = pad.horizontal + i as f64 * space + (space - thick) / 2.0;
                    let height = (point.value / max) * (h - 2.0 * pad.global);
                    let y = h - height - pad.global - point.padding.vertical;
                    (x, y, thick, height, x + thick / 2.0, y + height)
                }
                Direction::TopDown => {
                    let space = (w - 2.0 * pad.horizontal) / self.data.len() as f64;
                    let thick = space * 0.6;
                    let x = pad.horizontal + i as f64 * space + (space - thick) / 2.0;
                    let height = (point.value / max) * (h - 2.0 * pad.global);
                    let y = pad.global + point.padding.vertical;
                    (x, y, thick, height, x + thick / 2.0, y)
                }
                Direction::LeftRight => {
                    let space = (h - 2.0 * pad.vertical) / self.data.len() as f64;
                    let thick = space * 0.6;
                    let y = pad.vertical + i as f64 * space + (space - thick) / 2.0;
                    let width = (point.value / max) * (w - 2.0 * pad.global);
                    let x = pad.global + point.padding.horizontal;
                    (x, y, width, thick, x + width, y)
                }
                Direction::RightLeft => {
                    let space = (h - 2.0 * pad.vertical) / self.data.len() as f64;
                    let thick = space * 0.6;
                    let y = pad.vertical + i as f64 * space + (space - thick) / 2.0;
                    let width = (point.value / max) * (w - 2.0 * pad.global);
                    let x = w - pad.global - width - point.padding.horizontal;
                    (x, y, width, thick, x, y)
                }
            };
            // aplica sombra
            let sh = &point.shadow;
            ctx.set_shadow_blur(sh.blur);
            ctx.set_shadow_color(&sh.color.to_hex());
            ctx.set_shadow_offset_x(sh.offset_x);
            ctx.set_shadow_offset_y(sh.offset_y);
            // desenha barra
            ctx.set_global_alpha(bar_cfg.alpha);
            ctx.set_fill_style_str(&point.color.to_hex());
            if bar_cfg.border_radius > 0.0 {
                let radius = bar_cfg.border_radius.min(width / 2.0).min(height / 2.0);
                ctx.begin_path();
                ctx.move_to(x + radius, y);
                ctx.line_to(x + width - radius, y);
                ctx.quadratic_curve_to(x + width, y, x + width, y + radius);
                ctx.line_to(x + width, y + height - radius);
                ctx.quadratic_curve_to(x + width, y + height, x + width - radius, y + height);
                ctx.line_to(x + radius, y + height);
                ctx.quadratic_curve_to(x, y + height, x, y + height - radius);
                ctx.line_to(x, y + radius);
                ctx.quadratic_curve_to(x, y, x + radius, y);
                ctx.close_path();
                // preenche e contorna
                ctx.fill();
                if bar_cfg.stroke_color != Color::Transparent {
                    ctx.set_stroke_style_str(&bar_cfg.stroke_color.to_hex());
                    ctx.set_line_width(bar_cfg.line_width);
                    ctx.stroke();
                }
            } else {
                ctx.fill_rect(x, y, width, height);
            }
            if bar_cfg.stroke_color != Color::Transparent {
                ctx.set_stroke_style_str(&bar_cfg.stroke_color.to_hex());
                ctx.set_line_width(bar_cfg.line_width);
                ctx.stroke_rect(x, y, width, height);
            }
            // desenha texto
            let font_cfg = &point.font;
            ctx.set_font(&font_cfg.font);
            ctx.set_fill_style_str(&font_cfg.text_color.to_hex());

            match opts.direction {
                Direction::BottomUp => {
                    ctx.set_text_align("center");
                    ctx.set_text_baseline("bottom");
                }
                Direction::TopDown => {
                    ctx.set_text_align("center");
                    ctx.set_text_baseline("top");
                }
                Direction::LeftRight => {
                    ctx.set_text_align("right");
                    ctx.set_text_baseline("top");
                }
                Direction::RightLeft => {
                    ctx.set_text_align("left");
                    ctx.set_text_baseline("top");
                }
            }

            let label = if point.value > 999.0 {
                format!("{:.2e}", point.value)
            } else {
                format!("{:.0}", point.value)
            };
            let _ = ctx.fill_text(&label, tx, ty);
        }
    }
}
