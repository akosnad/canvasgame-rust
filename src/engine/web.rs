use super::*;
use crate::utils::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

pub struct WebEngine {
    ctx: web_sys::CanvasRenderingContext2d,
    screen: Screen,
    ftime: f64,
    pub world: crate::world::World,
}

unsafe impl Send for WebEngine {}

impl WebEngine {
    pub fn new(world: crate::world::World) -> Self {
        WebEngine {
            ctx: context(),
            //            screen: Screen::new(canvas().width() as usize, canvas().height() as usize),
            screen: Screen::new(500, 500),
            ftime: 0.,
            world: world,
        }
    }
}

impl Engine for WebEngine {
    fn engine_loop(&mut self) {
        let frame_start = js_sys::Date::now();

        self.world.tick();

        self.screen.render(&mut self.world);

        let mut bitmap: Vec<u8> = vec![0; self.screen.h * self.screen.w * 4];
        for i in (1..bitmap.len() + 1).step_by(4) {
            let x = (i / 4) % (self.screen.w);
            let y = (i / 4) / self.screen.h;
            bitmap[i - 1] = self.screen.pixels[y][x].r;
            bitmap[i + 0] = self.screen.pixels[y][x].g;
            bitmap[i + 1] = self.screen.pixels[y][x].b;
            bitmap[i + 2] = 255;
        }
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(bitmap.as_mut_slice()),
            self.screen.w as u32,
            self.screen.h as u32,
        )
        .unwrap();
        self.ctx.put_image_data(&image, 0., 0.).unwrap();

        self.ctx.set_fill_style(&"white".into());
        self.ctx.set_font(&"10px monospace");
        self.ctx
            .fill_text(&format!("ftime: {}", self.ftime), 10., 10.)
            .unwrap();
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
