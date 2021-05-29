use super::*;
use crate::utils::*;
use wasm_bindgen::prelude::*;

pub struct WebEngine {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    ftime: f64,
    pub world: crate::world::World,
}

unsafe impl Send for WebEngine {}

impl WebEngine {
    fn canvas_center(&self) -> (f64, f64) {
        (
            (self.canvas.width() / 2) as f64,
            (self.canvas.height() / 2) as f64,
        )
    }
}

impl Engine for WebEngine {
    fn init(world: crate::world::World) -> Self {
        WebEngine {
            canvas: canvas(),
            ctx: context(),
            ftime: 0.,
            world: world,
        }
    }
    fn engine_loop(&mut self) {
        let frame_start = js_sys::Date::now();

        self.world.tick();

        self.ctx.set_fill_style(&"black".into());
        self.ctx.fill_rect(
            0.,
            0.,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.ctx.set_fill_style(&"white".into());
        self.ctx.set_font(&"2em serif");
        self.ctx
            .fill_text(&format!("ftime: {}", self.ftime), 10., 100.)
            .unwrap();

        self.world.render(
            &self.ctx,
            self.canvas_center(),
            (self.canvas.width().into(), self.canvas.height().into()),
        );

        let frame_end = js_sys::Date::now();
        self.ftime = frame_end - frame_start;
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn key_down(e: web_sys::KeyboardEvent) {
    unsafe {
        match e.key().as_str() {
            "w" | "W" => MOVEMENT_KEYS.up = true,
            "a" | "A" => MOVEMENT_KEYS.left = true,
            "s" | "S" => MOVEMENT_KEYS.down = true,
            "d" | "D" => MOVEMENT_KEYS.right = true,
            " " | "Spacebar" => MOVEMENT_KEYS.jump = true,
            _ => {}
        }
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn key_up(e: web_sys::KeyboardEvent) {
    unsafe {
        match e.key().as_str() {
            "w" | "W" => MOVEMENT_KEYS.up = false,
            "a" | "A" => MOVEMENT_KEYS.left = false,
            "s" | "S" => MOVEMENT_KEYS.down = false,
            "d" | "D" => MOVEMENT_KEYS.right = false,
            " " | "Spacebar" => MOVEMENT_KEYS.jump = false,
            _ => {}
        }
    }
}
