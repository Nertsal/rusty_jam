use specs::{Entity, World};

mod orbit_launch;

pub use orbit_launch::OrbitLaunchAction;

pub trait Action: Sync + Send {
    fn perform(self: Box<Self>, world: &mut World, actor: Entity);
}

pub type Actions = Vec<Box<dyn Action>>;
