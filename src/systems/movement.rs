use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{Rigidbody, Transform},
    resources::Time,
};

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Rigidbody>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (rigidbodies, mut transforms, time): Self::SystemData) {
        for (rigidbody, transform) in (&rigidbodies, &mut transforms).join() {
            transform.position += rigidbody.velocity * time.delta_time;
        }
    }
}
