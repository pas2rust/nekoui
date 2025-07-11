use web_sys::CanvasRenderingContext2d;

pub trait Drawable {
    fn draw(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64);
}
