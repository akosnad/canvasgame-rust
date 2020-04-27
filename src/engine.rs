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

pub fn init_loop() {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut x = 0.;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut c = context();
        c.set_stroke_style(&"white".into());
        c.begin_path();
        c.move_to(0., 0.);
        c.line_to(x, x);
        c.close_path();
        c.stroke();
        x += 1.;
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}