#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use crate::world::Entity;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(feature = "no_std"))]
#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "no_std")]
pub mod bare;

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
    fn set(&self, idx: usize, color: (u8, u8, u8));

    fn fill_rect(&self, x: usize, y: usize, w: usize, h: usize, color: (u8, u8, u8)) {
        if x > WIDTH || x+w > WIDTH
        || y > HEIGHT || y+h > HEIGHT {
            return;
        }

        for i in x..x+w {
            for j in y..y+h {
                self.set(self.at(i, j), color);
            }
        }
    }

    fn render_entity(&self, entity: &Entity, offset: (f64, f64));
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