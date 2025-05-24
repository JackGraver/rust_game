use macroquad::prelude::info;

use crate::{
    ecs::{
        components::{GameObjectType, Position},
        world::World,
    },
    game::Game,
};

use crate::ecs::resource::rounds::*;

pub fn round_manager_system(world: &mut World, delta_time: f32) {
    if !world.new_round {
        let enemies_remaining = world
            .game_objs
            .iter()
            .filter(|(_, obj)| obj.obj_type == GameObjectType::Enemy)
            .count();

        if enemies_remaining <= 0 {
            world.round_num += 1;
            world.new_round = true;
        }
    }
}
