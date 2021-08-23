use specs::{Component, DenseVecStorage};

pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
