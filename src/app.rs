use eframe::egui::CtxRef;
use eframe::epi::Frame;

pub struct App;

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &CtxRef, frame: &Frame) {
    }

    fn name(&self) -> &str {
        "Hurdle"
    }
}