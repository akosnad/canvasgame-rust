#![feature(type_ascription)]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod engine;
pub mod world;

#[cfg(not(feature = "bare"))]
pub mod assets;

#[cfg(target_arch = "wasm32")]
mod wasm_utils;

#[cfg(target_arch = "wasm32")]
use {
    crate::wasm_utils::*, std::cell::RefCell, std::rc::Rc,
    wasm_bindgen::prelude::*,
    crate::assets::*,
};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(target_arch = "wasm32")]
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn run() {
    wasm_utils::set_panic_hook();
    let mut e = world::Entity::new();
    e.pos = world::Coord {
        x: 100.0,
        y: 200.0,
        z: 5.0,
    };
    let mut world = world::World::new();
    world.entities.push(e);

    log!("loading assets");
    let assets = load_assets().await.unwrap();
    log!("importing assets");
    import_assets(assets, &mut world).await.unwrap();

    let mut engine = engine::web::WebEngine::new(world);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        engine.engine_cycle();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}
