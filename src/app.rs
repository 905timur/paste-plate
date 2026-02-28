use eframe::App;
use egui::{Context, Vec2};
use crate::node::ImageNode;
use arboard::Clipboard;

pub struct PastePlateApp {
    pub nodes: Vec<ImageNode>,
    pub canvas_offset: Vec2,
    pub canvas_zoom: f32,
    pub clipboard: Option<Clipboard>,
    pub drag_state: Option<(usize, egui::Pos2)>,
    pub next_id: usize,
}

impl PastePlateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            nodes: Vec::new(),
            canvas_offset: Vec2::ZERO,
            canvas_zoom: 1.0,
            clipboard: Clipboard::new().ok(),
            drag_state: None,
            next_id: 0,
        }
    }
}

impl App for PastePlateApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        crate::input::handle_global_input(self, ctx);
        crate::canvas::draw_canvas(self, ctx);
    }
}
