use crate::world::{World, Entity};

pub mod web;

#[derive(Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Clone)]
struct Screen {
    pixels: Vec<Vec<Pixel>>,
    w: usize,
    h: usize,
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Self {
        let black = Pixel{
            r: 0,
            g: 0,
            b: 0,
        };
        let columns = vec![black; w];
        let rows = vec![columns; h];
        Self {
            pixels: rows,
            w: w,
            h: h
        }
    }

    fn center(&self) -> (f64, f64) {
        (
            (self.w as f64) / 2.,
            (self.h as f64) / 2.
        )
    }

    fn clear(&mut self) {
        let black = Pixel{
            r: 0,
            g: 0,
            b: 0,
        };
        let columns = vec![black; self.w];
        let rows = vec![columns; self.h];
        self.pixels = rows;
        for (i, rows) in self.pixels.iter_mut().enumerate() {
            for (j, col) in rows.iter_mut().enumerate() {
                if (j <= (self.w / 2) +5  && j >= (self.w /2) + 5)
                || (i <= (self.h / 2) +5 && i >= (self.h /2) +5) {
                    *col = Pixel{r: 255, g: 255, b: 255}
                }
            }
        }
    }

    fn fill(&mut self, x: usize, y: usize, w: usize, h: usize, new_pixel: Pixel) {
        for (i, rows) in self.pixels.iter_mut().enumerate() {
            if i >= y && i <= y+h {
                for (j, pixel) in rows.iter_mut().enumerate() {
                    if j >= x && j <= x+w {
                        *pixel = new_pixel.clone();
                    }
                }
            }
        }
    }

    fn render_entity(&mut self, entity: &Entity, offset: (f64, f64)) {
        let size_mult = 1. / (entity.hitbox.start.z / (entity.pos.z + entity.hitbox.start.z));
        let opacity = 255. * (entity.hitbox.start.z / (entity.pos.z + entity.hitbox.start.z));
        let center = self.center();

        let x = center.0 + entity.pos.x - offset.0 + (entity.hitbox.start.x * size_mult);
        let y = center.1 + entity.pos.y - offset.1 + (entity.hitbox.start.y * size_mult);
        let w = center.0 + entity.pos.x - offset.0 + (entity.hitbox.end.x * size_mult) - x;
        let h = center.1 + entity.pos.y - offset.1 + (entity.hitbox.end.y * size_mult) - y;
        self.fill(x as usize, y as usize, w as usize, h as usize, Pixel{r: 0, g: 0, b: 255});
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