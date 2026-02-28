mod app;
mod canvas;
mod node;
mod clipboard;
mod input;

use eframe::{egui, Result};

fn main() -> Result<()> {
    let mut viewport = egui::ViewportBuilder::default();
    
    if let Ok(icon_data) = std::fs::read("assets/icon.png") {
        if let Ok(img) = image::load_from_memory(&icon_data) {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            viewport = viewport.with_icon(
                std::sync::Arc::new(egui::IconData {
                    rgba: rgba.into_raw(),
                    width,
                    height,
                })
            );
        }
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Paste Plate",
        native_options,
        Box::new(|cc| Ok(Box::new(app::PastePlateApp::new(cc)))),
    )
}
