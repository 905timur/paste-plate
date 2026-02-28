use std::sync::Arc;
use egui::{ColorImage, Context, TextureOptions};
use image::{ImageBuffer, RgbaImage};
use crate::node::ImageNode;

pub fn paste_from_clipboard(app: &mut crate::app::PastePlateApp, ctx: &Context) {
    // Try arboard first
    let mut clipboard = match arboard::Clipboard::new() {
        Ok(cb) => cb,
        Err(e) => {
            println!("Paste failed: could not init clipboard: {:?}", e);
            return;
        }
    };

    println!("Attempting to get image from clipboard via arboard...");
    match clipboard.get_image() {
        Ok(image_data) => {
            println!("Got image from arboard: {}x{}", image_data.width, image_data.height);
            let width = image_data.width as u32;
            let height = image_data.height as u32;
            let rgba_image: RgbaImage = ImageBuffer::from_raw(width, height, image_data.bytes.into_owned())
                .expect("Failed to create RgbaImage from clipboard data");
            add_image_node(app, ctx, rgba_image);
            return;
        }
        Err(e) => {
            println!("arboard failed: {:?}", e);
        }
    }

    // Try wl-paste fallback for Wayland
    println!("Attempting wl-paste fallback...");
    match std::process::Command::new("wl-paste")
        .arg("--type")
        .arg("image/png")
        .output()
    {
        Ok(output) if output.status.success() => {
            println!("wl-paste succeeded, loading image...");
            match image::load_from_memory(&output.stdout) {
                Ok(img) => {
                    println!("Loaded image from wl-paste output");
                    add_image_node(app, ctx, img.into_rgba8());
                    return;
                }
                Err(err) => {
                    println!("Failed to decode wl-paste image: {:?}", err);
                }
            }
        }
        Ok(output) => {
            println!("wl-paste failed with status: {:?}", output.status);
        }
        Err(e) => {
            println!("wl-paste not available: {:?}", e);
        }
    }

    // Fallback: try text/uri parsing
    println!("Attempting text/uri fallback...");
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        if let Ok(text) = clipboard.get_text() {
            let first_line = text.lines().next().unwrap_or("").trim();
            
            let mut path = None;
            if first_line.starts_with("file://") {
                if let Ok(url) = url::Url::parse(first_line) {
                    if let Ok(file_path) = url.to_file_path() {
                        path = Some(file_path);
                    }
                }
            } else {
                let p = std::path::PathBuf::from(first_line);
                if p.is_absolute() {
                    path = Some(p);
                }
            }
            
            if let Some(p) = path {
                println!("Parsed file path from clipboard: {:?}", p);
                match image::open(&p) {
                    Ok(img) => {
                        println!("Successfully loaded image from {:?}", p);
                        add_image_node(app, ctx, img.into_rgba8());
                    }
                    Err(err) => {
                        println!("Failed to load image from {:?}: {:?}", p, err);
                    }
                }
            } else {
                println!("Clipboard text is not a valid recognized file path or URI");
            }
        }
    }
}

fn add_image_node(app: &mut crate::app::PastePlateApp, ctx: &Context, rgba_image: RgbaImage) {
    let width = rgba_image.width() as u32;
    let height = rgba_image.height() as u32;

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
