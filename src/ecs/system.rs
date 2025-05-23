pub fn movement_system(
    positions: &mut Vec<(usize, Position)>,
    velocities: &Vec<(usize, Velocity)>,
) {
    for (id, pos) in positions.iter_mut() {
        if let Some((_, vel)) = velocities.iter().find(|(vid, _)| vid == id) {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    }
}
