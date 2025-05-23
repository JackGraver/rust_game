use macroquad::window::{screen_height, screen_width};

use crate::ecs::{
    components::{GameObjectType, Position},
    world::World,
};

/// Handles moving all entities by updating Position component from Velocity component
/// - Removes bullets off-screen
pub fn movement_system(world: &mut World, delta: f32) {
    let mut bullets_to_remove = Vec::new();

    for (entity, vel) in world.velocities.iter() {
        if let Some(pos) = world.positions.get_mut(entity) {
            if world.game_objs[entity].obj_type != GameObjectType::Enemy {
                pos.x += vel.dx * delta;
                pos.y += vel.dy * delta;

                // Only pass what is needed to avoid borrow conflict
                if world.game_objs[entity].obj_type == GameObjectType::Bullet
                    && _check_offscreen_bullet(*entity, pos)
                {
                    bullets_to_remove.push(*entity);
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
