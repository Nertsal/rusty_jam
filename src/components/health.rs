use specs::{Component, DenseVecStorage};

pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}

impl Health {
    pub fn new(max: f32) -> Self {
        assert!(max > 0.0, "health must be positive!");
        Self { current: max, max }
    }

    pub fn change(&mut self, delta: f32) {
        self.current = (self.current + delta).clamp(0.0, self.max);
    }

    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0.0
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead()
    }
}
