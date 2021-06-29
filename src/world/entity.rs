use super::*;
use image::RgbImage;
use std::sync::Arc;

pub struct Entity {
    pub pos: Coord,
    pub velocity: Velocity,
    pub(in crate::world) in_air: bool,
    pub hitbox: Region,
    pub texture: Option<RgbImage>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            pos: Coord::origin(),
            hitbox: Region::default_hitbox(),
            velocity: Velocity::new(),
            in_air: false,
            texture: None,
        }
    }
    pub fn set_texture(&mut self, texture: Option<RgbImage>) {
        if let Some(bitmap) = texture {
            let (w, h) = (bitmap.width(), bitmap.height());
            self.hitbox.start.x = - (w as f64 / 2.);
            self.hitbox.start.y = - (h as f64 / 2.);
            self.hitbox.end.x = w as f64 / 2.;
            self.hitbox.end.y = h as f64 / 2.;

            self.texture = Some(bitmap);
            return;
        }
        
        self.texture = None;
    }
    pub fn tick(&mut self) {
        // Limit max velocity
        self.velocity.limit();

        // Simulate velocity falloff
        if self.velocity.to.x > 0. { self.velocity.to.x -= self.velocity.falloff.x }
        if self.velocity.to.x < 0. { self.velocity.to.x += self.velocity.falloff.x }
        
        if self.velocity.to.y > 0. { self.velocity.to.y -= self.velocity.falloff.y }
        if self.velocity.to.y < 0. { self.velocity.to.y += self.velocity.falloff.y }
        
        self.velocity.to.z -= self.velocity.falloff.z;
        
        // Apply movement
        self.pos = self.pos + self.velocity.to;

        if self.pos.z < 0. { self.pos.z = 0.; self.velocity.to.z = 0.; self.in_air = false; }
    }
}

