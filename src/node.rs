use egui::{Pos2, TextureHandle};
use image::RgbaImage;
use std::sync::Arc;

pub struct ImageNode {
    pub id: usize,
    pub texture: TextureHandle,
    pub pixel_data: Arc<RgbaImage>,
    pub pos: Pos2, // Center position in canvas space
    pub scale: f32, // 1.0 = original size
    pub z_order: usize,
    pub selected: bool,
}
