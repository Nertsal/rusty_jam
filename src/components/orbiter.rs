use specs::{Component, DenseVecStorage, Entity};

pub struct Orbiter {
    pub origin: Entity,
    pub distance: f32,
    pub speed: f32,
}

impl Component for Orbiter {
    type Storage = DenseVecStorage<Self>;
}
