use specs::{Entity, World};

use crate::actions::Actions;

mod player;

pub use player::PlayerActor;

pub trait Actor: Sync + Send {
    fn update(&mut self, actor: Entity, world: &World, delta_time: f32) -> Actions;
}
