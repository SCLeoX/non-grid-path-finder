use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::geometry::{Segment, Vec2};

pub struct Canvas {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Canvas { canvas, ctx }
    }
    pub fn html_canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }
    pub fn clear(&self) {
        self.ctx
            .clear_rect(0., 0., self.canvas.width().into(), self.canvas.height().into());
    }
    pub fn begin_path(&self) {
        self.ctx.begin_path();
    }
    pub fn segment(&self, segment: &Segment) {
        self.move_to(segment.p0);
        self.line_to(segment.p1);
    }
    pub fn move_to(&self, point: Vec2) {
        self.ctx.move_to(point.x, point.y);
    }
    pub fn line_to(&self, point: Vec2) {
        self.ctx.line_to(point.x, point.y);
    }
    pub fn circle(&self, center: Vec2, radius: f64) {
        self.ctx
            .arc(center.x, center.y, radius, 0., std::f64::consts::PI * 2.)
            .unwrap();
    }
    pub fn set_stroke_style(&self, style: &str) {
        self.ctx.set_stroke_style(&JsValue::from_str(style));
    }
    pub fn set_fill_style(&self, style: &str) {
        self.ctx.set_fill_style(&JsValue::from_str(style));
    }
    pub fn stroke(&self) {
        self.ctx.stroke();
    }
    pub fn fill(&self) {
        self.ctx.fill();
    }
}
