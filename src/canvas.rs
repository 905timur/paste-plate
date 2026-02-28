use egui::{Color32, Context, Rect, Stroke, Sense, Pos2};
use crate::app::PastePlateApp;

pub fn draw_canvas(app: &mut PastePlateApp, ctx: &Context) {
    let frame = egui::Frame::canvas(&ctx.style()).fill(Color32::from_rgb(30, 30, 30));
    
    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());
        
        let pointer_in_canvas = response.hover_pos();
        let scroll_delta = ctx.input(|i| i.smooth_scroll_delta);
        
        if ctx.input(|i| i.modifiers.ctrl) {
            let zoom_delta = scroll_delta.y * 0.01;
            if zoom_delta != 0.0 {
                let old_zoom = app.canvas_zoom;
                app.canvas_zoom = (app.canvas_zoom + zoom_delta).clamp(0.1, 10.0);
                
                if let Some(pos) = pointer_in_canvas {
                    let screen_pos = pos.to_vec2();
                    let canvas_pos = (screen_pos - app.canvas_offset) / old_zoom;
                    app.canvas_offset = screen_pos - canvas_pos * app.canvas_zoom;
                }
            }
        }
        
        if response.dragged_by(egui::PointerButton::Middle) || (ctx.input(|i| i.modifiers.command || i.key_down(egui::Key::Space)) && response.dragged_by(egui::PointerButton::Primary)) {
            app.canvas_offset += response.drag_delta();
        }
        
        app.nodes.sort_by_key(|n| n.z_order);
        
        let mut clicked_node = None;
        if response.clicked() || response.drag_started() {
            if let Some(pos) = pointer_in_canvas {
                for node in app.nodes.iter().rev() {
                    let screen_center = app.canvas_offset + node.pos.to_vec2() * app.canvas_zoom;
                    let size = node.texture.size_vec2() * node.scale * app.canvas_zoom;
                    let node_rect = Rect::from_center_size(screen_center.to_pos2(), size);
                    if node_rect.contains(pos) {
                        clicked_node = Some(node.id);
                        break;
                    }
                }
            }
        }
        
        if let Some(id) = clicked_node {
            if response.clicked() || response.drag_started() {
                for node in &mut app.nodes {
                    node.selected = node.id == id;
                }
                
                if response.clicked() {
                    let max_z = app.nodes.iter().map(|n| n.z_order).max().unwrap_or(0);
                    if let Some(node) = app.nodes.iter_mut().find(|n| n.id == id) {
                        node.z_order = max_z + 1;
                    }
                }
            }
            if response.drag_started() {
                 app.drag_state = Some((id, pointer_in_canvas.unwrap()));
            }
        } else if response.clicked() && !ctx.input(|i| i.key_down(egui::Key::Space)) {
            for node in &mut app.nodes {
                node.selected = false;
            }
        }
        
        if let Some((drag_id, _origin)) = app.drag_state {
            if response.dragged_by(egui::PointerButton::Primary) && !ctx.input(|i| i.key_down(egui::Key::Space)) {
                let drag_delta = response.drag_delta() / app.canvas_zoom;
                if let Some(node) = app.nodes.iter_mut().find(|n| n.id == drag_id) {
                    node.pos += drag_delta;
                }
            }
            if response.drag_stopped() {
                app.drag_state = None;
            }
        }
        
        let painter = ui.painter_at(rect);
        for node in &app.nodes {
            let screen_center = app.canvas_offset + node.pos.to_vec2() * app.canvas_zoom;
            let size = node.texture.size_vec2() * node.scale * app.canvas_zoom;
            let node_rect = Rect::from_center_size(screen_center.to_pos2(), size);
            
            painter.image(
                node.texture.id(),
                node_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
            
            if node.selected {
                painter.rect_stroke(node_rect, 0.0, Stroke::new(2.0, Color32::YELLOW));
            }
        }
        
        response.context_menu(|ui| {
            if ui.button("Paste").clicked() {
                crate::clipboard::paste_from_clipboard(app, ctx);
                ui.close_menu();
            }
        });
    });
}
