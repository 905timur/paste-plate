use egui::{Color32, Context, Sense, SidePanel, Stroke, TopBottomPanel, Vec2};
use crate::app::PastePlateApp;
use crate::tools::Tool;

/// Draws the tool ribbon - a vertical panel on the left side with tool buttons
pub fn draw_tool_ribbon(app: &mut PastePlateApp, ctx: &Context) {
    SidePanel::left("tool_ribbon")
        .width_range(40.0..=60.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Move tool button
                let move_btn = egui::Button::new("V")
                    .fill(if app.active_tool == Tool::Move {
                        egui::Color32::from_rgb(80, 80, 80)
                    } else {
                        egui::Color32::from_rgb(40, 40, 40)
                    })
                    .min_size(Vec2::new(40.0, 40.0));
                if ui.add(move_btn).clicked() {
                    app.active_tool = Tool::Move;
                }
                ui.add_space(4.0);

                // Transform tool button
                let transform_btn = egui::Button::new("T")
                    .fill(if app.active_tool == Tool::Transform {
                        egui::Color32::from_rgb(80, 80, 80)
                    } else {
                        egui::Color32::from_rgb(40, 40, 40)
                    })
                    .min_size(Vec2::new(40.0, 40.0));
                if ui.add(transform_btn).clicked() {
                    app.active_tool = Tool::Transform;
                }
                ui.add_space(4.0);

                // Selection tool button
                let selection_btn = egui::Button::new("S")
                    .fill(if app.active_tool == Tool::Selection {
                        egui::Color32::from_rgb(80, 80, 80)
                    } else {
                        egui::Color32::from_rgb(40, 40, 40)
                    })
                    .min_size(Vec2::new(40.0, 40.0));
                if ui.add(selection_btn).clicked() {
                    app.active_tool = Tool::Selection;
                }
            });
        });
}

/// Draws the layer ribbon - a horizontal panel at the bottom with layer thumbnails
pub fn draw_layer_ribbon(app: &mut PastePlateApp, ctx: &Context) {
    // Early return if no nodes - don't reserve any vertical space
    if app.nodes.is_empty() {
        return;
    }

    // Collect node IDs sorted by z_order descending (top layer first/leftmost)
    let mut node_ids: Vec<usize> = app.nodes.iter().map(|n| n.id).collect();
    node_ids.sort_by(|&a, &b| {
        let z_a = app.nodes.iter().find(|n| n.id == a).map(|n| n.z_order).unwrap_or(0);
        let z_b = app.nodes.iter().find(|n| n.id == b).map(|n| n.z_order).unwrap_or(0);
        z_b.cmp(&z_a)
    });

    // Collect z_order and selected state for each node (for display only)
    let node_info: std::collections::HashMap<usize, (usize, bool)> = app.nodes
        .iter()
        .map(|n| (n.id, (n.z_order, n.selected)))
        .collect();

    // Clone texture info - texture ID with size
    let texture_info: std::collections::HashMap<usize, (egui::TextureId, Vec2)> = app.nodes
        .iter()
        .map(|n| (n.id, (n.texture.id(), n.texture.size_vec2())))
        .collect();

    let node_ids_clone = node_ids.clone();
    let node_info_clone = node_info.clone();
    let texture_info_clone = texture_info;

    TopBottomPanel::bottom("layer_ribbon")
        .height_range(100.0..=140.0)
        .show(ctx, |ui| {
            egui::ScrollArea::horizontal()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for &node_id in &node_ids_clone {
                        let is_selected = node_info_clone.get(&node_id).map(|&(_, s)| s).unwrap_or(false);
                        let tex_info = texture_info_clone.get(&node_id).copied();

                        let (tex_id, tex_size) = match tex_info {
                            Some((id, size)) => (id, size),
                            None => continue,
                        };

                        ui.vertical(|ui| {
                            let thumb_size = Vec2::new(80.0, 80.0);

                            // Create image widget using texture ID with size tuple
                            let image = egui::Image::new((tex_id, tex_size))
                                .fit_to_exact_size(thumb_size)
                                .sense(Sense::click());

                            let response = ui.add(image);

                            // Add selection border if selected
                            if is_selected {
                                let rect = response.rect.expand(2.0);
                                ui.painter_at(rect).rect_stroke(
                                    rect,
                                    0.0,
                                    Stroke::new(2.0, Color32::YELLOW),
                                );
                            }

                            // Handle click to select only this node
                            if response.clicked() {
                                for n in &mut app.nodes {
                                    n.selected = n.id == node_id;
                                }
                            }

                            // Layer label
                            ui.label(format!("Layer {}", node_id));
                        });
                        ui.add_space(8.0);
                    }
                });
        });
}
