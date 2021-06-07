use super::*;

#[cfg(feature = "alloc")]
use spin::Mutex;

#[cfg(not(feature = "alloc"))]
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(vec![0; WIDTH * HEIGHT]);
}

pub struct BareEngine {
    pub world: crate::world::World,
}

impl BareEngine {
    pub fn new(world: crate::world::World) -> Self {
        Self {
            world: world,
        }
    }

    pub fn get_buf(&self) -> Vec<u8> {
        BUFFER.lock().clone()
    }

    pub fn engine_cycle(&mut self) {
        self.world.tick();
        self.render();
    }
}

impl Engine for BareEngine {
    fn clear(&self) {
        let mut buf = BUFFER.lock();
        *buf = vec![0; WIDTH * HEIGHT];
    }
    fn set(&self, _idx: usize, _color: (u8, u8, u8)) {
        //TODO
    }
    fn render(&mut self) {
        self.clear();
        self.world.scroll(self.center(), (WIDTH as f64, HEIGHT as f64));
        for entity in self.world.entities.iter() {
            self.render_entity(&entity, self.world.scroll);
        }
        self.render_entity(&self.world.player.entity, self.world.scroll);
    }
}