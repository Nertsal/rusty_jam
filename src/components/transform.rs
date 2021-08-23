use macroquad::prelude::Vec2;
use specs::{Component, VecStorage};

pub struct Transform {
    pub position: Vec2,
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
