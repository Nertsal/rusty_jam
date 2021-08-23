use macroquad::prelude::Vec2;
use specs::{Component, VecStorage};

pub struct RigidbodyController {
    pub target_velocity: Vec2,
    pub acceleration: f32,
}

impl Component for RigidbodyController {
    type Storage = VecStorage<Self>;
}
