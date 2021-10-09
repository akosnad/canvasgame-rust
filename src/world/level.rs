use super::*;

#[cfg(not(feature = "bare"))]
use image::RgbaImage;

#[derive(Clone)]
pub struct Level {
    pub boundary: Region,

    #[cfg(not(feature = "bare"))]
    pub bg_texture: Option<RgbaImage>,
}

impl Level {
    pub fn new(boundary: Region) -> Self {
        Self {
            boundary: boundary,

            #[cfg(not(feature = "bare"))]
            bg_texture: None,
        }
    }

    #[cfg(not(feature = "bare"))]
    pub fn set_bg_texture(&mut self, texture: Option<RgbaImage>) {
        if let Some(bitmap) = texture {
            self.bg_texture = Some(bitmap);
            return;
        }
        self.bg_texture = None;
    }
}
