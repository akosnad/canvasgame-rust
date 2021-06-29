use super::*;
use crate::wasm_utils::*;
use wasm_bindgen::prelude::*;

pub struct WebEngine {
    ctx: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
    ftime: f64,
    pub world: crate::world::World,
}

unsafe impl Send for WebEngine {}

impl WebEngine {
    pub fn new(world: crate::world::World) -> Self {
        WebEngine {
            ctx: context(),
            canvas: canvas(),
            ftime: 0.,
            world: world,
        }
    }
    fn dump(&self) {
        #[cfg(feature = "dump_log")]
        self.ctx
            .fill_text(
                &format!(
                    "pos: x: {:3.3} y: {:3.3} z: {:3.3} scroll: {:3.3}, {:3.3}",
                    self.world.player.entity.pos.x,
                    self.world.player.entity.pos.y,
                    self.world.player.entity.pos.z,
                    self.world.scroll.0,
                    self.world.scroll.1
                ),
                10.,
                30.,
            )
            .unwrap();
    }
    pub fn engine_cycle(&mut self) {
        let frame_start = js_sys::Date::now();

        self.world.tick();

        self.render();

        self.ctx.set_fill_style(&"white".into());
        self.ctx.set_font(&"10px monospace");
        self.ctx
            .fill_text(&format!("ftime: {}", self.ftime), 10., 10.)
            .unwrap();

        self.dump();

        let frame_end = js_sys::Date::now();
        self.ftime = frame_end - frame_start;
    }

}

impl Engine for WebEngine {
    fn width(&self) -> usize {
        self.canvas.width() as usize
    }
    fn height(&self) -> usize {
        self.canvas.height() as usize
    }

    fn clear(&self) {
        self.ctx.set_fill_style(&"black".into());
        self.ctx.fill_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
    }
    fn set_at(&self, _idx: usize, _color: (u8, u8, u8)) {
        // Unused in the web engine, since the rendering context uses x and y coords,
        // making this implementation unnecessary and would be slow.
        panic!("'set_at' should not be called on the web engine");
    }
    fn fill_rect(&self, x: usize, y: usize, w: usize, h: usize, color: (u8, u8, u8)) {
        self.ctx.set_fill_style(&format!("rgb({} {} {})", color.0, color.1, color.2).as_str().into());
        self.ctx.fill_rect(x as f64, y as f64, w as f64, h as f64);

    }
    fn render(&mut self) {
        self.clear();
        self.world.scroll(self.center(), (self.canvas.width() as f64, self.canvas.height() as f64));
        for entity in self.world.entities.iter() {
            self.render_entity(&entity, self.world.scroll);
        }
        self.render_entity(&self.world.player.entity, self.world.scroll);
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
