use macroquad::prelude::{is_key_down, vec2, KeyCode};
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::{
    components::{Player, RigidbodyController},
    constants::PLAYER_SPEED,
};

pub struct HandleInputSystem;

impl<'s> System<'s> for HandleInputSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, RigidbodyController>,
    );

    fn run(&mut self, (players, mut controllers): Self::SystemData) {
        for (_, controller) in (&players, &mut controllers).join() {
            let mut move_x = 0.0;
            if is_key_down(KeyCode::A) {
                move_x -= 1.0;
            }
            if is_key_down(KeyCode::D) {
                move_x += 1.0;
            }

            let mut move_y = 0.0;
            if is_key_down(KeyCode::S) {
                move_y -= 1.0;
            }
            if is_key_down(KeyCode::W) {
                move_y += 1.0;
            }

            controller.target_velocity = vec2(move_x, move_y).normalize_or_zero() * PLAYER_SPEED;
        }
    }
}
