use macroquad::{
    color::{GOLD, YELLOW},
    prelude::info,
};

use crate::ecs::{
    components::{GameObjectType, Position, Speed, Velocity},
    world::World,
};

// fn iter_bullets<'a>(&'a self) -> impl Iterator<Item = Entity> + 'a {
//     self.game_objs
//         .iter()
//         .filter(|(_, obj)| obj.obj_type == GameObjectType::Bullet)
//         .map(|(e, _)| *e)
// } for bullet in world.iter_bullets() { ... }

// struct Query<'a, T> {
//     iter: Box<dyn Iterator<Item = (Entity, &'a T)> + 'a>,
// } for (e, pos) in world.query::<Position>() { ... }

pub fn enemy_hit_system(world: &mut World) {
    let mut enemies_to_remove = Vec::new();
    let mut bullets_to_remove = Vec::new();
    let mut coins_to_spawn = Vec::new();

    // Clone necessary data to avoid borrow conflicts
    let positions = &world.positions;
    let game_objs = &world.game_objs;

    for (bullet_entity, bullet_obj) in game_objs.iter() {
        if bullet_obj.obj_type != GameObjectType::Bullet {
            continue;
        }

        // Get bullet position
        if let Some(bullet_pos) = positions.get(bullet_entity) {
            for (enemy_entity, enemy_obj) in game_objs.iter() {
                if enemy_obj.obj_type != GameObjectType::Enemy {
                    continue;
                }

                // Get enemy position
                if let Some(enemy_pos) = positions.get(enemy_entity) {
                    // Basic circular collision check (adjust radius as needed)
                    let dx = bullet_pos.x - enemy_pos.x;
                    let dy = bullet_pos.y - enemy_pos.y;
                    let dist_sq = dx * dx + dy * dy;
                    let collision_distance = bullet_pos.radius + enemy_pos.radius; // adjust as needed

                    if dist_sq < collision_distance * collision_distance {
                        enemies_to_remove.push(*enemy_entity);
                        bullets_to_remove.push(*bullet_entity);

                        // Stage a coin to spawn later
                        coins_to_spawn.push(Position {
                            x: enemy_pos.x,
                            y: enemy_pos.y,
                            radius: 5.0,
                            color: GOLD,
                        });
                    }
                }
            }
        }
    }

    // Remove enemies after the loop to avoid borrowing conflicts
    for entity in enemies_to_remove {
        world.remove_entity(entity);
    }
    for entity in bullets_to_remove {
        world.remove_entity(entity);
    }
    for pos in coins_to_spawn {
        let coin = world.spawn_entity();
        world
            .as_game_object(coin, GameObjectType::Coin)
            .with_position(coin, pos)
            .with_velocity(coin, Velocity { dx: 0.0, dy: 0.0 })
            .with_speed(coin, Speed(50.0));
    }
}
