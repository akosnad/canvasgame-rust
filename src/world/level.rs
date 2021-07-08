use super::*;
use image::RgbImage;

pub struct Level {
    pub boundary: Region,
    pub bg_texture: Option<RgbImage>,
}

impl Level {
    pub fn new(boundary: Region) -> Self {
        Self {
            boundary: boundary,
            bg_texture: None,
        }
    }
}