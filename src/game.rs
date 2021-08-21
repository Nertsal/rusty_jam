use macroquad::{
    camera::{set_camera, Camera2D},
    prelude::{screen_height, screen_width, vec2},
};
use specs::{Dispatcher, DispatcherBuilder, World, WorldExt};

use crate::resources::Time;

pub struct Game<'a, 'b> {
    world: World,
    update_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.insert(Time { delta_time: 1e-5 });
        world.insert(Camera2D::default());

        let mut update_dispatcher = DispatcherBuilder::new().build();
        update_dispatcher.setup(&mut world);

        Self {
            world,
            update_dispatcher,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let zoom = 0.0055;
        let game_camera = Camera2D {
            offset: vec2(0.0, 0.0),
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            ..Default::default()
        };
        set_camera(&game_camera);

        self.world.insert(game_camera);
        self.world.write_resource::<Time>().delta_time = delta_time;

        self.update_dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    pub fn fixed_update(&mut self, _delta_time: f32) {}

    pub fn draw(&mut self) {}
}
