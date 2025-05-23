use super::components::{
    Bullet, Enemy, GameObject, GameObjectType, Health, Position, Shooter, Velocity,
};
use super::resource::rounds::{RoundPhase, RoundState};
use std::collections::HashMap;

pub type Entity = usize;

pub struct World {
    pub round_state: RoundState,
    pub next_id: Entity,
    pub player_entity: Entity,
    pub game_objs: HashMap<Entity, GameObject>,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub health: HashMap<Entity, Health>,
    pub shooters: HashMap<Entity, Shooter>,
}

impl World {
    pub fn new() -> Self {
        Self {
            round_state: RoundState {
                phase: RoundPhase::Live,
            },
            next_id: 2,
            player_entity: 1,
            game_objs: HashMap::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            health: HashMap::new(),
            shooters: HashMap::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.health.remove(&entity);
        self.shooters.remove(&entity);
    }

    pub fn as_game_object(&mut self, entity: Entity, obj_type: GameObjectType) -> &mut Self {
        self.game_objs.insert(entity, GameObject { obj_type });
        self
    }

    pub fn with_position(&mut self, entity: Entity, pos: Position) -> &mut Self {
        self.positions.insert(entity, pos);
        self
    }

    pub fn with_velocity(&mut self, entity: Entity, vel: Velocity) -> &mut Self {
        self.velocities.insert(entity, vel);
        self
    }

    pub fn with_health(&mut self, entity: Entity, health: Health) -> &mut Self {
        self.health.insert(entity, health);
        self
    }

    pub fn with_shooter(&mut self, entity: Entity, shooter: Shooter) -> &mut Self {
        self.shooters.insert(entity, shooter);
        self
    }
}
