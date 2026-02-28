use std::sync::Arc;
use egui::{ColorImage, Context, TextureOptions};
use image::{ImageBuffer, RgbaImage};
use crate::node::ImageNode;

pub fn paste_from_clipboard(app: &mut crate::app::PastePlateApp, ctx: &Context) {
    let clipboard = match app.clipboard.as_mut() {
        Some(cb) => cb,
        None => return,
    };

    if let Ok(image_data) = clipboard.get_image() {
        let width = image_data.width as u32;
        let height = image_data.height as u32;
        let rgba_image: RgbaImage = ImageBuffer::from_raw(width, height, image_data.bytes.into_owned())
            .expect("Failed to create RgbaImage from clipboard data");

        let color_image = ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            rgba_image.as_raw(),
        );
        let texture = ctx.load_texture(
            format!("clipboard_tex_{}", app.next_id),
            color_image,
            TextureOptions::LINEAR,
        );

        let screen_center = ctx.screen_rect().center();
        let canvas_center = (screen_center.to_vec2() - app.canvas_offset) / app.canvas_zoom;
        let canvas_pos = egui::pos2(canvas_center.x, canvas_center.y);

        let z_order = app.nodes.iter().map(|n| n.z_order).max().unwrap_or(0) + 1;

        let node = ImageNode {
            id: app.next_id,
            texture,
            pixel_data: Arc::new(rgba_image),
            pos: canvas_pos,
            scale: 1.0,
            z_order,
            selected: true,
        };

        for n in &mut app.nodes {
            n.selected = false;
        }

        app.nodes.push(node);
        app.next_id += 1;
    }
}
