use super::*;

pub struct BareEngine {
    screen: Screen,
    pub world: crate::world::World,
}

impl BareEngine {
    pub fn new(world: crate::world::World) -> Self {
        Self {
            screen: Screen::new(640, 480),
            world: world,
        }
    }
}

impl Engine for BareEngine {
    fn engine_loop(&mut self) {
        self.world.tick();
        self.screen.render(&mut self.world);
    }
}