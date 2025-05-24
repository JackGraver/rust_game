use crate::ecs::{
    components::{GameObjectType, Position},
    world::World,
};

pub fn enemy_separation_system(world: &mut World, delta: f32) {
    // Step 1: Collect all enemy entities and their positions by cloning to avoid mutable borrow conflicts
    let enemies: Vec<_> = world
        .game_objs
        .iter()
        .filter(|(_, obj)| obj.obj_type == GameObjectType::Enemy)
        .map(|(entity, _)| *entity)
        .collect();

    // Clone positions so we can do calculations without borrowing conflicts
    let mut positions_snapshot: Vec<(usize, Position)> = enemies
        .iter()
        .filter_map(|entity| world.positions.get(entity).map(|pos| (*entity, *pos)))
        .collect();

    // Step 2: Calculate separation adjustments on the snapshot positions
    // We’ll store position deltas (offsets) to apply later
    let mut adjustments: Vec<(usize, f32, f32)> = Vec::with_capacity(enemies.len());
    for i in 0..positions_snapshot.len() {
        let (entity_a, pos_a) = positions_snapshot[i];
        let mut adjust_x = 0.0;
        let mut adjust_y = 0.0;

        for j in 0..positions_snapshot.len() {
            if i == j {
                continue;
            }
            let (entity_b, pos_b) = positions_snapshot[j];
            let dx = pos_b.x - pos_a.x;
            let dy = pos_b.y - pos_a.y;
            let dist = (dx * dx + dy * dy).sqrt();
            let min_dist = pos_a.radius + pos_b.radius;

            if dist > 0.0 && dist < min_dist {
                let overlap = min_dist - dist;
                let nx = dx / dist;
                let ny = dy / dist;

                // Accumulate separation push away from neighbors
                adjust_x -= nx * overlap;
                adjust_y -= ny * overlap;
            }
        }

        adjustments.push((entity_a, adjust_x, adjust_y));
    }

    // Step 3: Apply the adjustments back to the original positions mutably, one by one
    for (entity, adj_x, adj_y) in adjustments {
        if let Some(pos) = world.positions.get_mut(&entity) {
            pos.x += adj_x * 0.5; // optional factor to smooth movement
            pos.y += adj_y * 0.5;
        }
    }
}
