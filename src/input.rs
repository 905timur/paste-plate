use egui::Context;
use crate::app::PastePlateApp;

pub fn handle_global_input(app: &mut PastePlateApp, ctx: &Context) {
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::V)) {
        crate::clipboard::paste_from_clipboard(app, ctx);
    }
    
    if ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
        app.nodes.retain(|n| !n.selected);
    }
    
    if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
        export_canvas(app);
    }
}

fn export_canvas(app: &PastePlateApp) {
    if app.nodes.is_empty() {
        return;
    }
    
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    
    for node in &app.nodes {
        let half_w = (node.pixel_data.width() as f32 * node.scale) / 2.0;
        let half_h = (node.pixel_data.height() as f32 * node.scale) / 2.0;
        
        let node_min_x = node.pos.x - half_w;
        let node_min_y = node.pos.y - half_h;
        let node_max_x = node.pos.x + half_w;
        let node_max_y = node.pos.y + half_h;
        
        if node_min_x < min_x { min_x = node_min_x; }
        if node_min_y < min_y { min_y = node_min_y; }
        if node_max_x > max_x { max_x = node_max_x; }
        if node_max_y > max_y { max_y = node_max_y; }
    }
    
    let canvas_width = (max_x - min_x).ceil() as u32;
    let canvas_height = (max_y - min_y).ceil() as u32;
    
    if canvas_width == 0 || canvas_height == 0 {
        return;
    }
    
    let mut base_image = image::RgbaImage::new(canvas_width, canvas_height);
    
    let mut sorted_nodes: Vec<_> = app.nodes.iter().collect();
    sorted_nodes.sort_by_key(|n| n.z_order);
    
    for node in &sorted_nodes {
        let x = (node.pos.x - min_x - (node.pixel_data.width() as f32 * node.scale) / 2.0).round() as i64;
        let y = (node.pos.y - min_y - (node.pixel_data.height() as f32 * node.scale) / 2.0).round() as i64;
        
        let img_to_overlay = if node.scale != 1.0 {
            let new_w = (node.pixel_data.width() as f32 * node.scale) as u32;
            let new_h = (node.pixel_data.height() as f32 * node.scale) as u32;
            image::imageops::resize(node.pixel_data.as_ref(), new_w, new_h, image::imageops::FilterType::CatmullRom)
        } else {
            (*node.pixel_data).clone()
        };
        
        image::imageops::overlay(&mut base_image, &img_to_overlay, x, y);
    }
    
    if let Ok(home) = std::env::var("HOME") {
        let export_path = std::path::PathBuf::from(home).join("Pictures").join("paste-plate-export.png");
        let _ = base_image.save(&export_path);
    }
}
