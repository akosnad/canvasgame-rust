use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on a window")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("game-canvas")
        .expect("no game canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("canvas should have 2d context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("can't convert ctx")
}

pub struct Engine {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
    ftime: f64,
    pub world: super::world::World,
}

unsafe impl Send for Engine {}

impl Engine {
    pub fn new(world: super::world::World) -> Engine {
        Engine {
            canvas: canvas(),
            ctx: context(),
            ftime: 0.,
            world: world,
        }
    }
    fn render_loop(&mut self) {
        let frame_start = js_sys::Date::now();

        self.world.tick();

        self.ctx.set_fill_style(&"black".into());
        self.ctx.fill_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);

        self.ctx.set_fill_style(&"white".into());
        self.ctx.set_font(&"2em serif");
        self.ctx
            .fill_text(&format!("ftime: {}", self.ftime), 10., 100.)
            .unwrap();

        self.world.render(&self.ctx, self.canvas_center(), (self.canvas.width().into(), self.canvas.height().into()));

        let frame_end = js_sys::Date::now();
        self.ftime = frame_end - frame_start;
    }

    fn canvas_center(&self) -> (f64, f64) {
        (
            (self.canvas.width()  / 2) as f64,
            (self.canvas.height() / 2) as f64
        )
    }


}

pub fn init_loop() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        super::ENGINE.lock().unwrap().render_loop();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
}

pub struct MovementKeys {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool
}

pub static mut MOVEMENT_KEYS: MovementKeys = MovementKeys {
    up: false,
    down: false,
    left: false,
    right: false,
    jump: false,
};

#[wasm_bindgen]
pub fn key_down(e: web_sys::KeyboardEvent) {
    unsafe { match e.key().as_str() {
        "w"|"W" =>        { MOVEMENT_KEYS.up    = true },
        "a"|"A" =>        { MOVEMENT_KEYS.left  = true },
        "s"|"S" =>        { MOVEMENT_KEYS.down  = true },
        "d"|"D" =>        { MOVEMENT_KEYS.right = true },
        " "|"Spacebar" => { MOVEMENT_KEYS.jump  = true },
        _ => {}
    }}
}

#[wasm_bindgen]
pub fn key_up(e: web_sys::KeyboardEvent) {
    unsafe { match e.key().as_str() {
        "w"|"W" =>        { MOVEMENT_KEYS.up    = false },
        "a"|"A" =>        { MOVEMENT_KEYS.left  = false },
        "s"|"S" =>        { MOVEMENT_KEYS.down  = false },
        "d"|"D" =>        { MOVEMENT_KEYS.right = false },
        " "|"Spacebar" => { MOVEMENT_KEYS.jump  = false },
        _ => {}
    }}
}