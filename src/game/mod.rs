use macroquad::{
    camera::{set_camera, Camera2D},
    prelude::{screen_height, screen_width, vec2, Vec2},
};
use specs::{Builder, Dispatcher, DispatcherBuilder, Join, World, WorldExt};

use crate::{
    actors::PlayerActor, components::*, constants::*, physics::PhysicsMaterial, resources::*,
    systems::*,
};

mod draw;

pub struct Game<'a, 'b> {
    world: World,
    update_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();

        world.register::<Transform>();
        world.register::<Rigidbody>();
        world.register::<RigidbodyController>();
        world.register::<ColorComponent>();
        world.register::<Player>();
        world.register::<Friend>();
        world.register::<Enemy>();
        world.register::<Health>();
        world.register::<Projectile>();
        world.register::<ActorComponent>();
        world.register::<Orbiter>();

        world.insert(Time { delta_time: 1e-5 });
        world.insert(Camera2D::default());

        let mut update_dispatcher = DispatcherBuilder::new()
            .with(HandleInputSystem, "handle_input", &[])
            .with(OrbiterSystem, "orbiter", &[])
            .with(
                RigidbodyControlSystem,
                "rigidbody_control",
                &["handle_input", "orbiter"],
            )
            .with(MovementSystem, "movement", &["rigidbody_control"])
            .with(CollisionSystem, "collision", &["movement"])
            .with(DeathSystem, "death", &["collision"])
            .build();
        update_dispatcher.setup(&mut world);

        intialize_world(&mut world);

        Self {
            world,
            update_dispatcher,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let zoom = 0.01;
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
        self.perform_actions(delta_time);
    }

    pub fn fixed_update(&mut self, _delta_time: f32) {}

    fn perform_actions(&mut self, delta_time: f32) {
        let mut actions = Vec::new();
        for (entity, actor) in (
            &self.world.entities(),
            &mut self.world.write_component::<ActorComponent>(),
        )
            .join()
        {
            for action in actor.0.update(entity, &self.world, delta_time) {
                actions.push((action, entity));
            }
        }
        for (action, actor) in actions {
            action.perform(&mut self.world, actor);
        }
    }
}

fn intialize_world(world: &mut World) {
    world
        .create_entity()
        .with(Player)
        .with(Friend)
        .with(ActorComponent(Box::new(PlayerActor)))
        .with(Health::new(PLAYER_HP))
        .with(Transform {
            position: vec2(0.0, 0.0),
        })
        .with(Rigidbody {
            mass: PLAYER_MASS,
            radius: PLAYER_RADIUS,
            velocity: Vec2::ZERO,
            physics_material: PhysicsMaterial::default(),
        })
        .with(RigidbodyController {
            target_velocity: Vec2::ZERO,
            acceleration: PLAYER_ACCELERATION,
        })
        .with(ColorComponent {
            color: PLAYER_COLOR,
        })
        .build();

    world
        .create_entity()
        .with(Enemy)
        .with(Health::new(MELEE_HP))
        .with(Transform {
            position: vec2(10.0, 0.0),
        })
        .with(Rigidbody {
            mass: MELEE_MASS,
            radius: MELEE_RADIUS,
            velocity: Vec2::ZERO,
            physics_material: PhysicsMaterial::default(),
        })
        .with(RigidbodyController {
            target_velocity: Vec2::ZERO,
            acceleration: MELEE_ACCELERATION,
        })
        .with(ColorComponent { color: MELEE_COLOR })
        .build();
}
