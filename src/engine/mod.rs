use crate::world::{World, Entity};

pub mod web;

#[derive(Clone)]
struct Screen {
    ///A buffer with 3 * 8 bytes per pixel
    /// 
    /// A pixel consits of the base RGB colors, each being a u8 value.
    /// So, the vector is initialized like:
    /// `vec![0; w * h * 3]`
    pixels: Vec<u8>,
    w: usize,
    h: usize,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            pixels: vec![0; w * h * 3],
            w: w,
            h: h
        }
    }

    /// Returns the x and y coordinates on-screen based on the buffer index.
    fn coords(&self, idx: usize) -> (usize, usize) {
        (
            (idx / 3) % self.w,
            (idx / 3) / self.h
        )
    }

    fn at(&self, x: usize, y: usize) -> usize {
        (y * self.w + x) * 3
    }

    fn center(&self) -> (f64, f64) {
        (
            (self.w as f64) / 2.,
            (self.h as f64) / 2.
        )
    }

    fn clear(&mut self) {
        self.pixels = vec![0; self.w * self.h * 3];
        for i in (0..self.pixels.len()).step_by(3) {
            let (x, y) = self.coords(i);
            if x == self.w / 2 || y == self.h / 2{
                self.pixels[i + 0] = 128;
                self.pixels[i + 1] = 128;
                self.pixels[i + 2] = 128;
            }
        }
    }

    fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: (u8, u8, u8)) {
        if x > self.w || x+w > self.w
        || y > self.h || y+h > self.h {
            return;
        }

        for i in x..x+w {
            for j in y..y+h {
                let offset = self.at(i, j);
                self.pixels[offset + 0] = color.0;
                self.pixels[offset + 1] = color.1;
                self.pixels[offset + 2] = color.2;
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
        self.fill_rect(x as usize, y as usize, w as usize, h as usize, (0, 0, 255));
    }
    pub fn render(&mut self, world: &mut World) {
        self.clear();
        world.scroll(self.center(), (self.w as f64, self.h as f64));
        for entity in world.entities.iter_mut() {
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

pub trait Engine {
    fn engine_loop(&mut self);
}