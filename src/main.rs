// #![windows_subsystem = "windows"]
use macroquad::prelude::*;
use std::time::Instant;

mod actions;
mod actors;
mod components;
mod constants;
mod game;
mod physics;
mod resources;
mod systems;

use game::Game;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

#[macroquad::main("Rusty Jam")]
async fn main() {
    let mut game = Game::new();

    let mut debug_mode = false;
    let mut frame_time = 0.0;
    loop {
        if is_key_pressed(KeyCode::F6) {
            debug_mode = !debug_mode;
        }
        
        if debug_mode {
            println!("---- next frame ----");
        }
        let delta_time = get_frame_time();
        frame_time += delta_time;
        let time = Instant::now();
        game.update(delta_time);
        if debug_mode {
            println!("update: {}ms", time.elapsed().as_millis());
        }

        let time = Instant::now();
        let mut frames = 0;
        while frame_time >= FIXED_DELTA_TIME {
            game.fixed_update(FIXED_DELTA_TIME);
            frame_time -= FIXED_DELTA_TIME;
            frames += 1;
        }
        if debug_mode {
            println!(
                "fixed_update: {}ms / {} frames",
                time.elapsed().as_millis(),
                frames
            );
        }

        let time = Instant::now();
        game.draw();
        if debug_mode {
            println!("draw: {}ms", time.elapsed().as_millis());
        }
        next_frame().await;
    }
}
