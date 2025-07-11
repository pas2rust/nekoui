use crate::components::{
    adapters::{use_effect, use_node_ref},
    chart::dto::Drawable,
};
use kenzu::Builder;
use leptos::{
    html::Canvas,
    prelude::{Get, NodeRef, window},
};
use tailwind::components::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, wasm_bindgen::JsCast};

#[derive(Builder, Clone, Default, Debug)]
pub struct PaddingOptions {
    #[set(value = 3.0)]
    pub horizontal: f64,
    #[set(value = 1.5)]
    pub vertical: f64,
    #[set(value = 0.0)]
    pub global: f64,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct GridOptions {
    #[set(value = false)]
    pub show: bool,
    #[set(value = 40.0)]
    pub size: f64,
    #[set(value = Color::White)]
    pub color: Color,
    #[set(value = 0.5)]
    pub alpha: f64,
    #[set(value = 1.0)]
    pub line_width: f64,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct AxisOptions {
    #[set(value = true)]
    pub show_x: bool,
    #[set(value = true)]
    pub show_y: bool,
    #[set(value = Color::White)]
    pub color: Color,
    #[set(value = 1.0)]
    pub line_width: f64,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct FontOptions {
    #[set(value = "bold 8px Aria")]
    pub font: String,
    #[set(value = Color::White)]
    pub text_color: Color,
}

#[derive(Builder, Clone, Default, Debug)]
pub struct ShadowOptions {
    #[set(value = 0.0)]
    pub blur: f64,
    #[set(value = Color::Transparent)]
    pub color: Color,
    #[set(value = 0.0)]
    pub offset_x: f64,
    #[set(value = 0.0)]
    pub offset_y: f64,
}

#[derive(Builder, Clone, Debug, Default)]
pub struct ChartData {
    pub value: f64,
    #[set(value = Color::Blue(Shade::FiveHundred))]
    pub color: Color,
    pub padding: PaddingOptions,
    pub shadow: ShadowOptions,
    pub font: FontOptions,
}

pub fn use_canvas(chart: impl Drawable + 'static) -> NodeRef<Canvas> {
    let canvas_ref = use_node_ref();
    use_effect(move || {
        if let Some(canvas) = canvas_ref.get() {
            let canvas: HtmlCanvasElement = canvas;
            let dpr = window().device_pixel_ratio();
            let w = canvas.client_width() as f64;
            let h = canvas.client_height() as f64;
            canvas.set_width((w * dpr) as u32);
            canvas.set_height((h * dpr) as u32);
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            ctx.scale(dpr, dpr).unwrap();
            ctx.clear_rect(0.0, 0.0, w, h);
            chart.draw(&ctx, w, h);
        }
    });
    canvas_ref
}
