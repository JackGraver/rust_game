use crate::ecs::{components::GameObjectType, world::World};

pub fn coin_collect_system(world: &mut World) {
    let player_entity = world.player_entity;

    // Get player position & coin inventory component
    let player_pos = match world.positions.get(&player_entity) {
        Some(pos) => pos,
        None => return, // no player position? skip
    };

    let player_coin_inv = match world.coin_invs.get_mut(&player_entity) {
        Some(inv) => inv,
        None => return, // no inventory? skip
    };

    let mut coins_to_remove = Vec::new();

    for (entity, obj) in world.game_objs.iter() {
        if obj.obj_type == GameObjectType::Coin {
            if let Some(coin_pos) = world.positions.get(entity) {
                // Circle collision check
                let dx = player_pos.x - coin_pos.x;
                let dy = player_pos.y - coin_pos.y;
                let dist_sq = dx * dx + dy * dy;
                let collision_dist = player_pos.radius + coin_pos.radius + player_pos.radius;

                if dist_sq < collision_dist * collision_dist {
                    coins_to_remove.push(*entity);
                    player_coin_inv.collected += 1;
                }
            }
        }
    }

    // Remove collected coins from world
    for entity in coins_to_remove {
        world.remove_entity(entity);
    }
}
