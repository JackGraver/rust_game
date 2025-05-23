use crate::ecs::world::World;
use macroquad::prelude::*;

pub fn render_player_system(world: &World) {
    for (_, pos) in &world.positions {
        draw_circle(pos.x, pos.y, pos.radius, pos.color);
    }
}
