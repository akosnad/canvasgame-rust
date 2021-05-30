use super::*;
use gameloop::{FrameAction, GameLoop};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub struct NativeEngine {
    screen: Screen,
    pub window: Window,
    window_buffer: Vec<u32>,
    game_loop: GameLoop,
    pub world: crate::world::World,
}

impl NativeEngine {
    pub fn new(world: crate::world::World) -> Self {
        let mut window =
            Window::new("CanvasGame", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

        window.limit_update_rate(None);

        Self {
            screen: Screen::new(WIDTH, HEIGHT),
            world: world,
            window: window,
            game_loop: GameLoop::new(60, 10).expect("Failed to init game_loop"),
            window_buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    fn dump(&self, _interpolation: f64) {
        #[cfg(feature = "dump_log")]
        println!(
            "pos: x: {:3.3} y: {:3.3} z: {:3.3} scroll: {:3.3}, {:3.3}, frame interpolation: {:1.3}",
            self.world.player.entity.pos.x,
            self.world.player.entity.pos.y,
            self.world.player.entity.pos.z,
            self.world.scroll.0,
            self.world.scroll.1,
            _interpolation,
        );
    }
}

impl Engine for NativeEngine {
    fn engine_loop(&mut self) {
        loop {
            for action in self.game_loop.actions() {
                match action {
                    FrameAction::Render { interpolation } => {
                        self.screen.render(&mut self.world);

                        for i in 0..WIDTH * HEIGHT {
                            let j = i * 3;
                            let r = (self.screen.pixels[j + 0] as u32) << 16;
                            let g = (self.screen.pixels[j + 1] as u32) << 8;
                            let b = self.screen.pixels[j + 2] as u32;
                            self.window_buffer[i] = r + g + b;
                        }
                        self.dump(interpolation);
                        self.window
                            .update_with_buffer(&self.window_buffer, WIDTH, HEIGHT)
                            .unwrap();
                    }

                    FrameAction::Tick => {
                        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
                            return;
                        }
                        if let Some(keys) = self.window.get_keys() {
                            unsafe {
                                if keys.contains(&Key::W) { MOVEMENT_KEYS.up = true; }
                                else { MOVEMENT_KEYS.up = false; }

                                if keys.contains(&Key::S) { MOVEMENT_KEYS.down = true; }
                                else { MOVEMENT_KEYS.down = false; }

                                if keys.contains(&Key::A) { MOVEMENT_KEYS.left = true; }
                                else { MOVEMENT_KEYS.left = false; }

                                if keys.contains(&Key::D) { MOVEMENT_KEYS.right = true; }
                                else { MOVEMENT_KEYS.right = false; }

                                if keys.contains(&Key::Space) { MOVEMENT_KEYS.jump = true; }
                                else { MOVEMENT_KEYS.jump = false; }
                            }
                        }

                        self.world.tick();
                    }
                }
            }
        }
    }
}
