#![cfg(not(target_arch = "wasm32"))]
#![cfg(feature = "native")]

fn main() {
    use canvasgame_rust::engine::native::NativeEngine;
    use canvasgame_rust::world::*;

    let mut world = World::new();
    let mut e = Entity::new();
    e.pos = Coord {
        x: 100.0,
        y: 200.0,
        z: 5.0,
    };
    world.entities.push(e);
    let mut engine = NativeEngine::new(world);
    engine.engine_loop();
}
