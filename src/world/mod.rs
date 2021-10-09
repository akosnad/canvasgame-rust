pub mod base;
pub mod entity;
pub mod player;
pub mod level;

pub use base::*;
pub use entity::Entity;
pub use player::Player;
pub use level::Level;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[derive(Clone)]
pub struct World {
    pub player: Player,
    pub entities: Vec<Entity>,
    pub boundary: Region,
    /// Screen scroll amount relative to world origin (x, y)
    pub scroll: (f64, f64),
    /// Start scrolling screen when player hits given distance to canvas border (in percent)
    pub scroll_threshold: f64,
}

impl World {
    pub fn new() -> World {
        World {
            player: Player::new(),
            entities: Vec::new(),
            boundary: Region::default_boundary(),
            scroll: (0.0, 0.0),
            scroll_threshold: 0.85,
        }
    }
    pub fn scroll(&mut self, canvas_center: (f64, f64), canvas_size: (f64, f64)) {
        if self.player.entity.pos.x - self.scroll.0 + canvas_center.0 > canvas_size.0 * self.scroll_threshold {
            self.scroll.0 += self.player.entity.pos.x - self.scroll.0 + canvas_center.0 - canvas_size.0 * self.scroll_threshold;
        } else if self.player.entity.pos.x - self.scroll.0 + canvas_center.0 < canvas_size.0 * (1.0 - self.scroll_threshold) {
            self.scroll.0 += self.player.entity.pos.x - self.scroll.0 + canvas_center.0 - canvas_size.0 * (1.0 - self.scroll_threshold);
        }

        if self.player.entity.pos.y - self.scroll.1 + canvas_center.1 > canvas_size.1 * self.scroll_threshold {
            self.scroll.1 += self.player.entity.pos.y - self.scroll.1 + canvas_center.1 - canvas_size.1 * self.scroll_threshold;
        } else if self.player.entity.pos.y - self.scroll.1 + canvas_center.1 < canvas_size.1 * (1.0 - self.scroll_threshold) {
            self.scroll.1 += self.player.entity.pos.y - self.scroll.1 + canvas_center.1 - canvas_size.1 * (1.0 - self.scroll_threshold);
        }
    }
    pub fn tick(&mut self) {
        // FIXME: do all this without cloning and copying, thus more efficiently

        let collision_partners: Vec<Region> = self.entities.iter().map(|e| { e.absolute_pos() }).collect();

        self.player.tick(collision_partners.clone());

        for entity in self.entities.iter_mut() {
            let self_region = collision_partners.iter().position(|r| { *r == entity.absolute_pos() }).unwrap();
            let mut regions_without_self = collision_partners.clone();
            assert!(regions_without_self.remove(self_region) == entity.absolute_pos());

            entity.tick(regions_without_self);
        }
    }
}