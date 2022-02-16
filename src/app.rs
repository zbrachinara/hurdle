use eframe::egui::{CtxRef, Vec2};
use eframe::epi::Frame;

pub struct App {
    size: Vec2,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: Vec2::new(width as f32, height as f32),
        }
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {}

    fn name(&self) -> &str {
        "Hurdle"
    }

    fn max_size_points(&self) -> Vec2 {
        self.size
    }
}
