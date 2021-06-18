use super::*;

pub struct Entity {
    pub pos: Coord,
    pub velocity: Velocity,
    pub(in crate::world) in_air: bool,
    pub hitbox: Region,
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            pos: Coord::origin(),
            hitbox: Region::default_hitbox(),
            velocity: Velocity::new(),
            in_air: false,
        }
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

