use crate::ecs::{
    components::{GameObjectType, Position},
    world::World,
};

pub fn player_death_system(world: &World) -> bool {
    let player_entity = world.player_entity;
    let player_pos = match world.positions.get(&player_entity) {
        Some(pos) => pos,
        None => return false, // no player position, nothing to do
    };

    for (entity, obj) in world.game_objs.iter() {
        if obj.obj_type != GameObjectType::Enemy {
            continue;
        }

        if let Some(enemy_pos) = world.positions.get(entity) {
            // Calculate squared distance between player and enemy
            let dx = enemy_pos.x - player_pos.x;
            let dy = enemy_pos.y - player_pos.y;
            let dist_sq = dx * dx + dy * dy;

            // Collision distance (sum of radii)
            let collision_distance = player_pos.radius + enemy_pos.radius;

            if dist_sq < collision_distance * collision_distance {
                return true;
            }
        }
    }
    false
}
