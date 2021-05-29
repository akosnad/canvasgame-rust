#![feature(type_ascription)]

mod utils;
mod engine;
mod world;

use crate::engine::Engine;
use wasm_bindgen::prelude::*;
use crate::utils::*;
use std::cell::RefCell;
use std::rc::Rc;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() {
    utils::set_panic_hook();
    let mut e = world::Entity::new();
    e.pos = world::Coord {
        x: 100.0, y: 200.0, z: 5.0,
    };
    let mut world = world::World::new();
    world.entities.push(e);

    let mut engine = engine::web::WebEngine::new(world);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        engine.engine_loop();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
