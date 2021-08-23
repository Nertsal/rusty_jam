use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::{
    components::{Enemy, Friend, Health, Rigidbody, Transform},
    physics,
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Rigidbody>,
        ReadStorage<'s, Friend>,
        ReadStorage<'s, Enemy>,
        WriteStorage<'s, Health>,
    );

    fn run(
        &mut self,
        (entities, mut transforms, mut rigidbodies, friends, enemies, mut healths): Self::SystemData,
    ) {
        let mut collisions = Vec::new();
        for (friend, _, friend_transform, friend_rigidbody) in
            (&entities, &friends, &transforms, &rigidbodies).join()
        {
            for (enemy, _, enemy_transform, enemy_rigidbody) in
                (&entities, &enemies, &transforms, &rigidbodies).join()
            {
                if let Some(collision_info) = physics::collision_info(
                    friend_rigidbody,
                    friend_transform,
                    enemy_rigidbody,
                    enemy_transform,
                ) {
                    collisions.push((friend, enemy, collision_info));
                }
            }
        }

        for (entity, other, (collision, hit_info)) in collisions {
            let transform = transforms.get_mut(entity).unwrap();
            transform.position += collision.normal * collision.penetration;

            let body = rigidbodies.get_mut(entity).unwrap();
            body.velocity += hit_info.hit_self * collision.normal;
            if let Some(health) = healths.get_mut(entity) {
                health.change(-hit_info.hit_self);
            }

            let other_body = rigidbodies.get_mut(other).unwrap();
            other_body.velocity -= hit_info.hit_other * collision.normal;
            if let Some(health) = healths.get_mut(other) {
                health.change(-hit_info.hit_other);
            }
        }
    }
}
