use eframe::egui::{CtxRef, Vec2};
use eframe::epi::{Frame};

pub struct App;

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {}

    fn name(&self) -> &str {
        "Hurdle"
    }

    fn max_size_points(&self) -> Vec2 {
        Vec2::new(10000f32, 10000f32)
    }
}