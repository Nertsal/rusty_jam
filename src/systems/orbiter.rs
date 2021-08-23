use macroquad::prelude::vec2;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::components::{Orbiter, Rigidbody, RigidbodyController, Transform};

pub struct OrbiterSystem;

impl<'s> System<'s> for OrbiterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Orbiter>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Rigidbody>,
        WriteStorage<'s, RigidbodyController>,
    );

    fn run(
        &mut self,
        (entities, orbiters, mut transforms, rigidbodies, mut rigidbody_controllers): Self::SystemData,
    ) {
        for (entity, orbiter, controller) in
            (&entities, &orbiters, &mut rigidbody_controllers).join()
        {
            let origin_pos = transforms.get(orbiter.origin).unwrap().position;
            let origin_velocity = rigidbodies.get(orbiter.origin).unwrap().velocity;

            let position = transforms.get(entity).unwrap().position;
            if let Some(direction) = (position - origin_pos).try_normalize() {
                let transform = transforms.get_mut(entity).unwrap();
                transform.position = origin_pos + direction * orbiter.distance;

                let target = vec2(direction.y, -direction.x) * 5.0 + position;
                // let target_pos = origin_pos + direction * orbiter.distance;
                let target_dir = target - origin_pos;
                let angle = direction.angle_between(target_dir).abs();
                let speed = angle.min(0.2) / 0.2;
                let direction = vec2(direction.y, -direction.x);
                let signum = direction.dot(target_dir).signum();
                let direction = direction * signum * speed;
                controller.target_velocity =
                    //((target_pos - position).clamp_length_max(1.0) + direction) * orbiter.speed
                    direction * orbiter.speed
                        + origin_velocity;
            }
        }
    }
}
