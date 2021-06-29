#![cfg(feature = "native")]

use image::io::Reader;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>>{
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

    let player_texture = Reader::open("./player.png")?.decode()?;
    world.player.entity.set_texture(Some(player_texture.to_rgb8()));

    let mut engine = NativeEngine::new(world);
    engine.engine_loop();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Game exited with error: {}", e);
    }
}
