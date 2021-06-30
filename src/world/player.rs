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
    pub(in crate::world) fn tick(&mut self) {
        unsafe {
            if MOVEMENT_KEYS.up    { self.entity.velocity.to.y -= self.entity.velocity.falloff.y * 2.; }
            if MOVEMENT_KEYS.down  { self.entity.velocity.to.y += self.entity.velocity.falloff.y * 2.; }
            if MOVEMENT_KEYS.left  { self.entity.velocity.to.x -= self.entity.velocity.falloff.x * 2.; }
            if MOVEMENT_KEYS.right { self.entity.velocity.to.x += self.entity.velocity.falloff.x * 2.; }
            if MOVEMENT_KEYS.jump && !self.entity.in_air { self.entity.velocity.to.z += self.entity.velocity.max.z; self.entity.in_air = true; }
        }
        self.entity.tick();
    }
}
