use super::*;
use crate::engine::{MOVEMENT_KEYS};

#[derive(Clone)]
pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub(in crate::world) fn new() -> Player {
        Player {
            entity: Entity::new(),
        }
    }
    pub(in crate::world) fn tick(&mut self, collide_with: Vec<Region>) {
        unsafe {
            if MOVEMENT_KEYS.up    { self.entity.vel.to.y -= self.entity.vel.falloff.y * 2.; }
            if MOVEMENT_KEYS.down  { self.entity.vel.to.y += self.entity.vel.falloff.y * 2.; }
            if MOVEMENT_KEYS.left  { self.entity.vel.to.x -= self.entity.vel.falloff.x * 2.; }
            if MOVEMENT_KEYS.right { self.entity.vel.to.x += self.entity.vel.falloff.x * 2.; }
            if MOVEMENT_KEYS.jump && !self.entity.in_air { self.entity.vel.to.z += self.entity.vel.max.z; self.entity.in_air = true; }
        }
        self.entity.tick(collide_with);
    }
}
