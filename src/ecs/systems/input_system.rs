use crate::ecs::world::World;
use macroquad::input::{is_key_down, KeyCode};

/// Handles WASD input for moving player
/// - Makes sure player doesnt move off-screen
pub fn player_input_system(world: &mut World) {
    let speed = 250.0;

    match (
        world.velocities.get_mut(&world.player_entity),
        world.positions.get_mut(&world.player_entity),
    ) {
        (Some(velocity), Some(position)) => {
            velocity.dx = 0.0;
            velocity.dy = 0.0;

            if is_key_down(KeyCode::W) && position.y - position.radius > 0.0 {
                velocity.dy -= speed;
            }
            if is_key_down(KeyCode::S) && position.y + position.radius < 800.0 {
                velocity.dy += speed;
            }
            if is_key_down(KeyCode::A) && position.x - position.radius > 0.0 {
                velocity.dx -= speed
            }
            if is_key_down(KeyCode::D) && position.x + position.radius < 1200.0 {
                velocity.dx += speed;
            }
        }
        _ => {}
    }
}
