use macroquad::prelude::{is_key_pressed, KeyCode, YELLOW};

use crate::{
    actions::{Actions, OrbitLaunchAction},
    physics::PhysicsMaterial,
};

use super::Actor;

pub struct PlayerActor;

impl Actor for PlayerActor {
    fn update(
        &mut self,
        _actor: specs::Entity,
        _world: &specs::World,
        _delta_time: f32,
    ) -> Actions {
        let mut actions: Actions = vec![];
        if is_key_pressed(KeyCode::F) {
            actions.push(Box::new(OrbitLaunchAction {
                is_friend: true,
                distance: 15.0,
                speed: 50.0,
                acceleration: 5.0,
                mass: 2.5,
                radius: 2.0,
                physics_material: PhysicsMaterial::default(),
                color: YELLOW,
            }));
        }
        actions
    }
}
