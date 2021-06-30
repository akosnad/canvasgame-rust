use super::*;
use gameloop::{FrameAction, GameLoop};
use minifb::{Key, Window, WindowOptions};
use std::sync::Mutex;

const INITIAL_WIDTH: usize = 640;
const INITIAL_HEIGHT: usize = 480;

lazy_static::lazy_static! {
    static ref WINDOW_BUFFER: Mutex<Vec<u32>> = Mutex::new(vec![0; INITIAL_WIDTH * INITIAL_HEIGHT]);
}

pub struct NativeEngine {
    pub world: crate::world::World,
    pub window: Window,
    game_loop: GameLoop,
}

impl NativeEngine {
    pub fn new(world: crate::world::World) -> Self {
        let mut window =
            Window::new(
                "CanvasGame",
                INITIAL_WIDTH,
                INITIAL_HEIGHT, 
                WindowOptions {
                    resize: true,
                    .. WindowOptions::default()
                }
            ).unwrap();

        window.limit_update_rate(None);

        Self {
            world: world,
            window: window,
            game_loop: GameLoop::new(60, 10).expect("Failed to init game_loop"),
        }
    }

    fn dump(&self, _interpolation: f64) {
        #[cfg(feature = "dump_log")]
        println!(
            "x: {:3.3} y: {:3.3} z: {:3.3} scroll: {:3.3} {:3.3} frame interpolation: {:1.3} w: {} h: {}",
            self.world.player.entity.pos.x,
            self.world.player.entity.pos.y,
            self.world.player.entity.pos.z,
            self.world.scroll.0,
            self.world.scroll.1,
            _interpolation,
            self.width(),
            self.height(),
        );
    }

    pub fn engine_loop(&mut self) {
        let mut needs_render = false;
        let mut interpolation = 0.0;
        loop {
            if needs_render {
                needs_render = false;
                self.clear();
                self.world.scroll(self.center(), (self.width() as f64, self.height() as f64));

                // FIXME: this clone is nasty
                let current_world = self.world.clone();
                self.render_world(&current_world);

                self.dump(interpolation);
                let buf = WINDOW_BUFFER.lock().unwrap();
                self.window
                    .update_with_buffer(&buf, self.width(), self.height())
                    .unwrap();
            }

            for action in self.game_loop.actions() {
                match action {
                    FrameAction::Render { interpolation: i } => {
                        needs_render = true;
                        interpolation = i;
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

impl Engine for NativeEngine {
    #[inline]
    fn width(&self) -> usize {
        self.window.get_size().0
    }
    #[inline]
    fn height(&self) -> usize {
        self.window.get_size().1
    }

    fn clear(&self) {
        let mut buf = WINDOW_BUFFER.lock().unwrap();
        *buf = vec![0; self.width() * self.height()];
    }

    fn set_at(&self, idx: usize, color: (u8, u8, u8)) {
        let r = (color.0 as u32) << 16;
        let g = (color.1 as u32) << 8;
        let b =  color.2 as u32;
        let mut buf = WINDOW_BUFFER.lock().unwrap();
        buf[idx] = r + g + b;
    }
}
