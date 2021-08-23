use macroquad::prelude::Color;
use specs::{Component, VecStorage};

pub struct ColorComponent {
    pub color: Color,
}

impl Component for ColorComponent {
    type Storage = VecStorage<Self>;
}
