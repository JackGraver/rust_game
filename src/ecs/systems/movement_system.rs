use macroquad::{
    prelude::info,
    window::{screen_height, screen_width},
};

use crate::ecs::{
    components::{GameObjectType, Position},
    world::World,
};

/// Handles moving all entities by updating Position component from Velocity component
/// - Removes bullets off-screen
pub fn movement_system(world: &mut World, delta: f32) {
    let mut bullets_to_remove = Vec::new();

    // mutable velocities to modify vel fields
    let positions = &mut world.positions;
    let velocities = &mut world.velocities;
    let game_objs = &world.game_objs;
    let player_entity = world.player_entity;

    let player_pos = positions.get(&player_entity).cloned();

    for (entity, vel) in velocities.iter_mut() {
        if let Some(pos) = positions.get_mut(entity) {
            match game_objs[entity].obj_type {
                GameObjectType::Player | GameObjectType::Bullet => {
                    pos.x += vel.dx * delta;
                    pos.y += vel.dy * delta;

                    if let GameObjectType::Bullet = game_objs[entity].obj_type {
                        if _check_offscreen_bullet(*entity, pos) {
                            bullets_to_remove.push(*entity);
                        }
                    }
                }

                GameObjectType::Enemy => {
                    if let Some(player_pos) = player_pos {
                        let dx = player_pos.x - pos.x;
                        let dy = player_pos.y - pos.y;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist > 0.0 {
                            let speed = 150.0;
                            let new_dx = dx / dist * speed;
                            let new_dy = dy / dist * speed;

                            pos.x += new_dx * delta;
                            pos.y += new_dy * delta;
                        }
                    }
                }

                GameObjectType::Coin => {
                    if let Some(player_pos) = player_pos {
                        if let Some(speed) = world.speeds.get_mut(entity) {
                            // Accelerate speed
                            speed.0 += 600.0 * delta;

                            let dx = player_pos.x - pos.x;
                            let dy = player_pos.y - pos.y;
                            let dist = (dx * dx + dy * dy).sqrt();

                            if dist > 0.0 {
                                let dir_x = dx / dist;
                                let dir_y = dy / dist;

                                vel.dx = dir_x * speed.0;
                                vel.dy = dir_y * speed.0;
                            }

                            // Also update position with current velocity
                            pos.x += vel.dx * delta;
                            pos.y += vel.dy * delta;
                        }
                    }
                }
            }
        }
    }

    for entity in bullets_to_remove {
        world.remove_entity(entity);
    }
}

fn _check_offscreen_bullet(entity: usize, pos: &Position) -> bool {
    let screen_width = screen_width();
    let screen_height = screen_height();

    pos.x < 0.0 || pos.x > screen_width || pos.y < 0.0 || pos.y > screen_height
}
