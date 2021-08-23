use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::{
    components::{Rigidbody, RigidbodyController},
    resources::Time,
};

pub struct RigidbodyControlSystem;

impl<'s> System<'s> for RigidbodyControlSystem {
    type SystemData = (
        ReadStorage<'s, RigidbodyController>,
        WriteStorage<'s, Rigidbody>,
        Read<'s, Time>,
    );

    fn run(&mut self, (rigidbody_controllers, mut rigidbodies, time): Self::SystemData) {
        for (controller, body) in (&rigidbody_controllers, &mut rigidbodies).join() {
            body.velocity += (controller.target_velocity - body.velocity)
                * controller.acceleration
                * time.delta_time;
        }
    }
}
