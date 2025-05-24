use std::f32::consts::E;

use crate::ecs::systems::coin_collect_system;
use crate::ecs::systems::coin_collect_system::coin_collect_system;
use crate::ecs::systems::enemy_hit_system::enemy_hit_system;
use crate::ecs::systems::enemy_seperation_system::enemy_separation_system;
use crate::ecs::systems::health_system::health_system;
use crate::ecs::systems::input_system::player_input_system;
use crate::ecs::systems::movement_system::movement_system;
use crate::ecs::systems::new_round_system::new_round_system;
use crate::ecs::systems::player_death_system::player_death_system;
use crate::ecs::systems::render_system::render_player_system;
use crate::ecs::systems::round_manager_system::round_manager_system;
use crate::ecs::systems::shooter_system::shooter_system;

use macroquad::prelude::{collections::storage::get, *};
use macroquad::time::get_frame_time;

use crate::ecs::{
    components::{GameObjectType, Health, Position, Shooter, Velocity},
    world::World,
};

pub struct Game {
    pub world: World,
    pub is_game_over: bool,
    avg_fps: i32,
    fps_count: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();

        // Spawn player entity
        let player = world.player_entity;
        world
            .as_game_object(player, GameObjectType::Player)
            .with_position(
                player,
                Position {
                    x: screen_width() / 2.0,
                    y: screen_height() / 2.0,
                    radius: 10.0,
                    color: PINK,
                },
            )
            .with_velocity(player, Velocity { dx: 0.0, dy: 0.0 })
            .with_health(
                player,
                Health {
                    current: 100,
                    max: 100,
                },
            )
            .with_shooter(player, Shooter { cooldown: 0.1 })
            .with_coins_inventory(player);

        Game {
            world,
            is_game_over: false,
            avg_fps: get_fps(),
            fps_count: 1,
        }
    }

    pub fn update(&mut self) {
        if player_death_system(&self.world) {
            info!("game over");
        } else {
            let delta = get_frame_time();

            round_manager_system(&mut self.world, delta);
            player_input_system(&mut self.world);
            movement_system(&mut self.world, delta);
            health_system(&mut self.world);
            shooter_system(&mut self.world, delta);
            enemy_hit_system(&mut self.world);
            coin_collect_system(&mut self.world);
            new_round_system(&mut self.world);
            enemy_separation_system(&mut self.world, delta);
        }
    }

    pub fn draw(&mut self) {
        self.draw_debug();
        render_player_system(&self.world);
    }

    pub fn draw_debug(&mut self) {
        self.fps_count += 1;
        self.avg_fps += get_fps();
        draw_text(
            &format!("FPS: {}", self.avg_fps / self.fps_count),
            10.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Entities: {}", self.world.positions.len()),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        let collected = self
            .world
            .coin_invs
            .get(&self.world.player_entity)
            .map(|inv| inv.collected)
            .unwrap_or(0);

        draw_text(
            &format!("Collected: {}", collected),
            10.0,
            100.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Round: {}", self.world.round_num),
            10.0,
            140.0,
            30.0,
            WHITE,
        );
    }
}

// use crate::entities::{bullet::Bullet, collision, enemy::Enemy, player::Player};
// use macroquad::prelude::*;
// pub struct Game {
//     pub player: Player,
//     pub bullets: Vec<Bullet>,
//     pub enemies: Vec<Enemy>,
//     pub is_game_over: bool,
// }

// impl Game {
//     pub fn new() -> Self {
//         Game {
//             player: Player::new(),
//             bullets: Vec::new(),
//             enemies: Enemy::generate_enemies(15),
//             is_game_over: false,
//         }
//     }

//     pub fn update(&mut self) {
//         if self.is_game_over {
//             self.draw_end_screen();
//             return;
//         }
//         self.check_collisions();

//         let delta = get_frame_time();

//         // update player
//         self.player.update(delta);

//         // add any new bullets
//         if let Some((vx, vy)) = self.player.try_fire() {
//             let bullet = Bullet::new(self.player.x, self.player.y, vx, vy);
//             self.bullets.push(bullet);
//         }

//         // update bullets (if any)
//         for bullet in &mut self.bullets {
//             bullet.update();
//         }

//         // update alive enemies
//         for enemy in &mut self.enemies {
//             enemy.update(delta, &self.player);
//         }
//     }

//     pub fn check_collisions(&mut self) {
//         if collision::player_enemy(&self.player, &self.enemies) {
//             self.is_game_over = true;
//             return;
//         }

//         collision::bullet_enemy_collisions(&mut self.bullets, &mut self.enemies);
//     }

//     pub fn draw_end_screen(&mut self) {
//         clear_background(WHITE);
//         let text = "Game Over. Press [SPACE] to play again.";
//         let font_size = 30.;
//         let text_size = measure_text(text, None, font_size as _, 1.0);

//         draw_text(
//             text,
//             screen_width() / 2. - text_size.width / 2.,
//             screen_height() / 2. + text_size.height / 2.,
//             font_size,
//             DARKGRAY,
//         );

//         if is_key_down(KeyCode::Space) {
//             self.player.x = 100.0;
//             self.player.y = 100.0;

//             self.enemies = Enemy::generate_enemies(15);

//             self.is_game_over = false;
//         }
//     }

//     pub fn draw(&self) {
//         self.player.draw();

//         for bullet in &self.bullets {
//             bullet.draw();
//         }

//         for enemy in &self.enemies {
//             enemy.draw();
//         }
//     }
// }
