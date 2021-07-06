use wasm_bindgen::{prelude::*, JsCast};
use image::DynamicImage;
use serde::Deserialize;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on a window")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn canvas() -> web_sys::HtmlCanvasElement {
    document()
        .get_element_by_id("game-canvas")
        .expect("no game canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}

pub fn context() -> web_sys::CanvasRenderingContext2d {
    canvas()
        .get_context("2d")
        .expect("canvas should have 2d context")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("can't convert ctx")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//#[link(wasm_import_module = "../www/assets.js")]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = window)]
    async fn load_asset_index_inner() -> JsValue;

    #[wasm_bindgen(js_namespace = window)]
    async fn load_asset_file_inner(ptr: *const u8, len: usize) -> JsValue;
}

#[wasm_bindgen]
pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[derive(Deserialize, Debug)]
struct PtrLen {
    ptr: usize,
    len: usize
}

fn jsval_to_ptrlen(val: JsValue) -> (*mut u8, usize) {
    let obj: PtrLen = val.into_serde().unwrap();
    (obj.ptr as *mut u8, obj.len)
}

fn asset_data(val: JsValue) -> Vec<u8> {
    let (ptr, len) = jsval_to_ptrlen(val);
    let data = unsafe { Vec::from_raw_parts(ptr, len, len) };
    data
}


pub async fn load_asset_index() -> Vec<u8> {
    let val = load_asset_index_inner().await;
    asset_data(val)
}

pub async fn load_asset_file(ptr: *const u8, len: usize) -> Vec<u8> {
    let val = load_asset_file_inner(ptr, len).await;
    asset_data(val)
}

pub fn load_image_from_array(buf: &[u8]) -> DynamicImage {
    let result = image::load_from_memory(buf);
    match result {
        Err(e) => {
            panic!("failed to load image: {}", e);
        }
        Ok(img) => { return img; }
    }
}
