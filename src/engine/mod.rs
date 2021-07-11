#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use crate::world::{Entity, World};
use image::RgbaImage;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "bare")]
pub mod bare;

pub type Pixel = (u8, u8, u8);

pub trait Engine {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    #[inline]
    fn at(&self, x: usize, y: usize) -> usize {
        y * self.width() + x
    }
    #[inline]
    fn center(&self) -> (f64, f64) {
        (
            (self.width()  as f64) / 2.,
            (self.height() as f64) / 2.
        )
    }

    fn clear(&mut self);
    fn set_at(&mut self, idx: usize, pixel: Pixel);

    /// If the texture has no alpha channel, it is safe to implement this funciton as no-op, since
    /// we call `set_at()` always in that case.
    fn set_at_with_opacity(&mut self, idx: usize, pixel: Pixel, opacity: f64);

    #[inline]
    fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.set_at(self.at(x, y), pixel)
    }
    #[inline]
    fn set_with_opacity(&mut self, x: usize, y: usize, pixel: Pixel, opacity: f64) {
        self.set_at_with_opacity(self.at(x, y), pixel, opacity)
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, pixel: Pixel) {
        let (gw, gh) = (self.width(), self.height());
        if x > gw || x+w > gw
        || y > gh || y+h > gh {
            return;
        }

        for i in x..x+w {
            for j in y..y+h {
                self.set(i, j, pixel);
            }
        }
    }

    fn fill_bitmap(&mut self, bitmap: &RgbaImage, x: usize, y: usize) {
        for i in 0..bitmap.width() {
            for j in 0..bitmap.height() {
                let p = bitmap.get_pixel(i, j);
                let opacity = p[3] as f64 / 255.;
                if opacity == 1. {
                    self.set(x + i as usize, y + j as usize, (p[0], p[1], p[2]));
                } else {
                    self.set_with_opacity(x + i as usize, y + j as usize, (p[0], p[1], p[2]), opacity);
                }
            }
        }
    }
    
    fn render_entity(&mut self, entity: &Entity, offset: (f64, f64)) {
        let size_mult = 1. / (entity.hitbox.start.z / (entity.pos.z + entity.hitbox.start.z));
        let center = self.center();

        let x = center.0 + entity.pos.x - offset.0 + (entity.hitbox.start.x * size_mult);
        let y = center.1 + entity.pos.y - offset.1 + (entity.hitbox.start.y * size_mult);
        let w = center.0 + entity.pos.x - offset.0 + (entity.hitbox.end.x * size_mult) - x;
        let h = center.1 + entity.pos.y - offset.1 + (entity.hitbox.end.y * size_mult) - y;

        if x < 0. || y < 0. {
            return;
        }

        if let Some(bitmap) = &entity.texture {
            self.fill_bitmap(bitmap, x as usize, y as usize);
            return;
        }

        self.fill_rect(x as usize, y as usize, w as usize, h as usize, (255, 0, 255)); // Missing texture
    }

    fn render_world(&mut self, world: &World) {
        for entity in world.entities.iter() {
            self.render_entity(&entity, world.scroll);
        }
        self.render_entity(&world.player.entity, world.scroll);
    }
}

pub struct MovementKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool
}

pub static mut MOVEMENT_KEYS: MovementKeys = MovementKeys {
    up: false,
    down: false,
    left: false,
    right: false,
    jump: false,
};