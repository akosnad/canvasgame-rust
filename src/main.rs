#![cfg(not(target_arch = "wasm32"))]


fn main() {
    use minifb::Key;
    use canvasgame_rust::engine::Engine;

    let world = canvasgame_rust::world::World::new();
    let mut engine = canvasgame_rust::engine::native::NativeEngine::new(world);
    while engine.window.is_open() && !engine.window.is_key_down(Key::Escape) {
        engine.engine_loop();
    }

}