#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use crate::world::Entity;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "bare")]
pub mod bare;

// TODO: Use a proper window resizing mechanism
//       and use these as an initial window size?
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub trait Engine {
    /// Returns the x and y coordinates on-screen based on the buffer index.
    fn coords(&self, idx: usize) -> (usize, usize) {
        (
            (idx / 3) % WIDTH,
            (idx / 3) / HEIGHT
        )
    }
    fn at(&self, x: usize, y: usize) -> usize {
        y * WIDTH + x
    }
    fn center(&self) -> (f64, f64) {
        (
            (WIDTH as f64) / 2.,
            (HEIGHT as f64) / 2.
        )
    }

    fn clear(&self);
    fn set_at(&self, idx: usize, color: (u8, u8, u8));

    fn set(&self, x: usize, y: usize, color: (u8, u8, u8)) {
        self.set_at(self.at(x, y), color);
    }

    fn fill_rect(&self, x: usize, y: usize, w: usize, h: usize, color: (u8, u8, u8)) {
        if x > WIDTH || x+w > WIDTH
        || y > HEIGHT || y+h > HEIGHT {
            return;
        }

        for i in x..x+w {
            for j in y..y+h {
                self.set(i, j, color);
            }
        }
    }
    fn render_entity(&self, entity: &Entity, offset: (f64, f64)) {
        let size_mult = 1. / (entity.hitbox.start.z / (entity.pos.z + entity.hitbox.start.z));
        let center = self.center();

        let x = center.0 + entity.pos.x - offset.0 + (entity.hitbox.start.x * size_mult);
        let y = center.1 + entity.pos.y - offset.1 + (entity.hitbox.start.y * size_mult);
        let w = center.0 + entity.pos.x - offset.0 + (entity.hitbox.end.x * size_mult) - x;
        let h = center.1 + entity.pos.y - offset.1 + (entity.hitbox.end.y * size_mult) - y;

        if x < 0. || y < 0. {
            return;
        }
        self.fill_rect(x as usize, y as usize, w as usize, h as usize, (0, 0, 255));
    }
    fn render(&mut self);
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