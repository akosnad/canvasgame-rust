#![feature(type_ascription)]

mod utils;
mod engine;
mod world;

use wasm_bindgen::prelude::*;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref ENGINE: Arc<Mutex<engine::Engine>> = Arc::new(Mutex::new(
        engine::Engine::new(world::World::new())));
}

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
    let mut e = world::Entity::new();
    e.pos = world::Coord {
        x: 100.0, y: 200.0, z: 0.0,
    };
    ENGINE.lock().unwrap().world.entities.push(e);
    engine::init_loop();
}
