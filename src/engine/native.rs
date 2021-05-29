use super::*;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub struct NativeEngine {
    screen: Screen,
    pub window: Window,
    window_buffer: Vec<u32>,
    pub world: crate::world::World,
}

impl NativeEngine {
    pub fn new(world: crate::world::World) -> Self {
        let mut window =
            Window::new("CanvasGame", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        //Limit to ~60 FPS
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            screen: Screen::new(WIDTH, HEIGHT),
            world: world,
            window: window,
            window_buffer: vec![0; WIDTH * HEIGHT],
        }
    }
}

impl Engine for NativeEngine {
    fn engine_loop(&mut self) {
        if let Some(keys) =  self.window.get_keys() {
            unsafe {
                if keys.contains(&Key::W) { MOVEMENT_KEYS.up = true; }
                else { MOVEMENT_KEYS.up = false;}

                if keys.contains(&Key::S) { MOVEMENT_KEYS.down = true; }
                else { MOVEMENT_KEYS.down = false;}

                if keys.contains(&Key::A) { MOVEMENT_KEYS.left = true; }
                else { MOVEMENT_KEYS.left = false;}

                if keys.contains(&Key::D) { MOVEMENT_KEYS.right = true; }
                else { MOVEMENT_KEYS.right = false;}

                if keys.contains(&Key::Space) { MOVEMENT_KEYS.jump = true; }
                else { MOVEMENT_KEYS.jump = false;}
            }
        }

        self.world.tick();
        self.screen.render(&mut self.world);

        for i in 0..WIDTH * HEIGHT {
            let j = i * 3;
            let r = (self.screen.pixels[j + 0] as u32) << 16;
            let g = (self.screen.pixels[j + 1] as u32) << 8;
            let b =  self.screen.pixels[j + 2] as u32;
            self.window_buffer[i] = r + g + b;
        }
        self.window.update_with_buffer(&self.window_buffer, WIDTH, HEIGHT).unwrap();
    }
}