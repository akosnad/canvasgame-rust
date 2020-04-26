mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn render_loop(ctx: web_sys::CanvasRenderingContext2d) {
    ctx.set_stroke_style(&"white".into());

    // Wall
    ctx.stroke_rect(75., 140., 150., 110.);

    // Door
    ctx.fill_rect(130., 190., 40., 60.);

    // Roof
    ctx.begin_path();
    ctx.move_to(50., 140.);
    ctx.line_to(150., 60.);
    ctx.line_to(250., 140.);
    ctx.close_path();
    ctx.stroke();
}