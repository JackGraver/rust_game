use crate::ecs::{
    components::{Bullet, Position, Velocity},
    world::World,
};
use macroquad::{
    color::RED,
    input::{is_mouse_button_down, mouse_position, MouseButton},
};

pub fn shooter_system(world: &mut World, delta: f32) {
    let mut new_bullets = Vec::new();

    // First pass: gather bullets to spawn
    for (&entity, shooter) in world.shooters.iter_mut() {
        if shooter.cooldown > 0.0 {
            shooter.cooldown -= delta;
        }

        if shooter.cooldown <= 0.0 && is_mouse_button_down(MouseButton::Left) {
            if let Some(&position) = world.positions.get(&entity) {
                let (mouse_x, mouse_y) = mouse_position();

                // Calculate direction vector from player to mouse
                let dx = mouse_x - position.x;
                let dy = mouse_y - position.y;

                // Calculate the length (distance)
                let distance = (dx * dx + dy * dy).sqrt();

                if distance != 0.0 {
                    // Normalize direction and multiply by speed
                    let speed = 1500.0; // You can tweak this
                    let vx = dx / distance * speed;
                    let vy = dy / distance * speed;

                    shooter.cooldown = 0.1;

                    // Store the bullet's initial position and velocity to add later
                    new_bullets.push((
                        Position {
                            x: position.x,
                            y: position.y,
                            radius: 4.0,
                            color: RED,
                        },
                        Velocity { dx: vx, dy: vy },
                    ));
                }
            }
        }
    }

    // Second pass: spawn bullet entities
    for (pos, vel) in new_bullets {
        let bullet_entity = world.spawn_entity();
        world
            .as_game_object(
                bullet_entity,
                crate::ecs::components::GameObjectType::Bullet,
            )
            .with_position(bullet_entity, pos)
            .with_velocity(bullet_entity, vel);
    }
}
