use crate::ecs::world::World;

pub fn health_system(world: &mut World) {
    for (_, health) in world.health.iter_mut() {
        if health.current == 0 {
            health.current = health.max;
        }
    }
}
