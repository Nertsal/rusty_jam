use specs::{Component, NullStorage};

#[derive(Default)]
pub struct Friend;

impl Component for Friend {
    type Storage = NullStorage<Self>;
}
