use macroquad::prelude::Color;
use specs::{Builder, WorldExt};

use crate::{
    components::{
        ColorComponent, Enemy, Friend, Health, Orbiter, Rigidbody, RigidbodyController, Transform,
    },
    physics::PhysicsMaterial,
};

use super::Action;

pub struct OrbitLaunchAction {
    pub is_friend: bool,
    pub distance: f32,
    pub speed: f32,
    pub acceleration: f32,
    pub mass: f32,
    pub radius: f32,
    pub physics_material: PhysicsMaterial,
    pub color: Color,
}

impl Action for OrbitLaunchAction {
    fn perform(self: Box<Self>, world: &mut specs::World, actor: specs::Entity) {
        let position = world
            .read_component::<Transform>()
            .get(actor)
            .unwrap()
            .position;
        let velocity = world
            .read_component::<Rigidbody>()
            .get(actor)
            .unwrap()
            .velocity;

        let entity = world
            .create_entity()
            .with(Orbiter {
                origin: actor,
                distance: self.distance,
                speed: self.speed,
            })
            .with(Health::new(1.0))
            .with(Transform { position })
            .with(Rigidbody {
                mass: self.mass,
                radius: self.radius,
                velocity,
                physics_material: self.physics_material,
            })
            .with(RigidbodyController {
                target_velocity: velocity,
                acceleration: self.acceleration,
            })
            .with(ColorComponent { color: self.color });

        let entity = if self.is_friend {
            entity.with(Friend)
        } else {
            entity.with(Enemy)
        };

        entity.build();
    }
}
