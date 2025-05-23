use macroquad::prelude::info;

use crate::ecs::{
    components::{GameObjectType, Position},
    world::World,
};

use crate::ecs::resource::rounds::*;

pub fn round_manager_system(world: &mut World, delta_time: f32) {
    let round = &mut world.round_state;

    match round.phase {
        RoundPhase::Warmup => {
            // round.time_remaining -= delta_time;
            // if round.time_remaining <= 0.0 {
            //     round.phase = RoundPhase::Live;
            //     round.time_remaining = 120.0;
            //     round.freeze_time = false;
            //     println!("Round {} started!", round.current_round);

            //     // Optionally reset player positions, etc.
            // }
        }
        RoundPhase::Live => {
            // round.time_remaining -= delta_time;
            // if round.time_remaining <= 0.0 || check_win_condition(world) {
            //     round.phase = RoundPhase::Ended;
            //     round.freeze_time = true;
            //     round.time_remaining = 5.0;
            //     println!("Round {} ended!", round.current_round);
            // }
        }
        RoundPhase::Ended => {
            // round.time_remaining -= delta_time;
            // if round.time_remaining <= 0.0 {
            //     round.phase = RoundPhase::Warmup;
            //     round.current_round += 1;
            //     round.time_remaining = 15.0;
            //     println!("Next round starting...");
            // }
        }
    }
}
