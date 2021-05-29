use crate::world::World;

pub mod web;

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
    fn init(world: World) -> Self;
    fn engine_loop(&mut self);
}

