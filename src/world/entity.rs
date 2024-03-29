use super::*;

#[cfg(not(feature = "bare"))]
use image::RgbaImage;

#[derive(Clone)]
pub struct Entity {
    pub pos: Coord,
    pub vel: Velocity,
    pub(in crate::world) in_air: bool,
    pub hitbox: Region,

    #[cfg(not(feature = "bare"))]
    pub texture: Option<RgbaImage>,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            pos: Coord::origin(),
            hitbox: Region::default_hitbox(),
            vel: Velocity::new(),
            in_air: false,

            #[cfg(not(feature = "bare"))]
            texture: None,
        }
    }

    /// Returns the `Entity`'s region relative to the world
    pub(in crate::world) fn absolute_pos(&self) -> Region {
        Region {
            start: self.pos + self.hitbox.start,
            end: self.pos + self.hitbox.end,
        }
    }

    #[cfg(not(feature = "bare"))]
    pub fn set_texture(&mut self, texture: Option<RgbaImage>) {
        if let Some(bitmap) = texture {
            let (w, h) = (bitmap.width(), bitmap.height());
            self.hitbox.start.x = -(w as f64 / 2.);
            self.hitbox.start.y = -(h as f64 / 2.);
            self.hitbox.end.x = w as f64 / 2.;
            self.hitbox.end.y = h as f64 / 2.;

            self.texture = Some(bitmap);
            return;
        }
        self.texture = None;
    }

    fn collide(&mut self, collide_with: Vec<Region>) {
        for other in collide_with.iter() {
            const COLLISION_THRESHOLD: f64 = 4.;

            if self.absolute_pos().overlaps(*other) {
                if self.vel.to.z != 0. {
                    if self.pos.z + self.hitbox.end.z - COLLISION_THRESHOLD >= other.start.z {
                        continue;
                    } else {
                        //self.pos.z = other.start.y - self.hitbox.end.z;
                        self.pos.z -= self.vel.to.z;
                        self.vel.to.z = 0.;
                        self.in_air = false;
                    }
                }

                if self.vel.to.x > 0.
                    && self.pos.x + self.hitbox.end.x - COLLISION_THRESHOLD <= other.start.x
                {
                    self.pos.x = other.start.x - self.hitbox.end.x;
                    self.vel.to.x = 0.;
                } else if self.vel.to.x < 0.
                    && self.pos.x + self.hitbox.start.x + COLLISION_THRESHOLD >= other.end.x
                {
                    self.pos.x = other.end.x - self.hitbox.start.x;
                    self.vel.to.x = 0.;
                }

                if self.vel.to.y > 0.
                    && self.pos.y + self.hitbox.end.y - COLLISION_THRESHOLD <= other.start.y
                {
                    self.pos.y = other.start.y - self.hitbox.end.y;
                    self.vel.to.y = 0.;
                } else if self.vel.to.y < 0.
                    && self.pos.y + self.hitbox.start.y + COLLISION_THRESHOLD >= other.end.y
                {
                    self.pos.y = other.end.y - self.hitbox.start.y;
                    self.vel.to.y = 0.;
                }
            }
        }
    }

    pub fn tick(&mut self, collide_with: Vec<Region>) {
        self.vel.limit();

        self.vel.do_falloff();

        // Apply movement
        self.pos = self.pos + self.vel.to;

        if self.pos.z < 0. {
            self.pos.z = 0.;
            self.vel.to.z = 0.;
            self.in_air = false;
        }

        self.collide(collide_with);
    }
}
