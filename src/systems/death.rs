use specs::{Entities, Join, ReadStorage, System};

use crate::components::Health;

pub struct DeathSystem;

impl<'s> System<'s> for DeathSystem {
    type SystemData = (Entities<'s>, ReadStorage<'s, Health>);

    fn run(&mut self, (entities, healths): Self::SystemData) {
        let mut deaths = Vec::new();
        for (entity, health) in (&entities, &healths).join() {
            if health.is_dead() {
                deaths.push(entity);
            }
        }
        for death in deaths {
            entities.delete(death).unwrap();
        }
    }
}
