#![cfg(not(target_arch = "wasm32"))]


fn main() {
    use canvasgame_rust::engine::Engine;

    let world = canvasgame_rust::world::World::new();
    let mut engine = canvasgame_rust::engine::native::NativeEngine::new(world);
    engine.engine_loop();
}