use crate::ecs::{
    components::{GameObjectType, Health, Position, Velocity},
    world::World,
};
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub fn new_round_system(world: &mut World) {
    if world.new_round {
        for i in 0..15 {
            let entity = world.spawn_entity();
            world
                .as_game_object(entity, GameObjectType::Enemy)
                .with_velocity(entity, Velocity { dx: 0.0, dy: 0.0 })
                .with_position(
                    entity,
                    Position {
                        x: gen_range(30.0, 1170.0),
                        y: (30.0),
                        radius: (10.0),
                        color: (BLUE),
                    },
                )
                .with_health(
                    entity,
                    Health {
                        current: 100,
                        max: 100,
                    },
                );
        }
        world.new_round = false;
    }
}
