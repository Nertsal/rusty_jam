use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}
