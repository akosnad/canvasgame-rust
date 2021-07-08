#![cfg(feature = "native")]

use std::error::Error;
use futures::executor::block_on;

async fn run() -> Result<(), Box<dyn Error>> {
    use canvasgame_rust::{engine::native::NativeEngine, world::*, assets::*};

    let mut world = World::new();
    let mut e = Entity::new();
    e.pos = Coord {
        x: 100.0,
        y: 200.0,
        z: 5.0,
    };
    e.hitbox.start.z = 4.;
    world.entities.push(e);

    let assets = load_assets()?;
    import_assets(assets, &mut world).await?;

    let mut engine = NativeEngine::new(world);
    engine.engine_loop();

    Ok(())
}

fn main() {
    let future = run();
    if let Err(e) = block_on(future) {
        println!("Game exited with error: {}", e);
    }
}
