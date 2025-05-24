use super::components::{
    Bullet, CoinInv, Enemy, GameObject, GameObjectType, Health, Position, Shooter, Speed, Velocity,
};
use super::resource::rounds::{RoundPhase, RoundState};
use std::collections::HashMap;

pub type Entity = usize;

pub struct World {
    pub round_num: i16,
    pub new_round: bool,
    pub next_id: Entity,
    pub player_entity: Entity,
    pub game_objs: HashMap<Entity, GameObject>,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub speeds: HashMap<Entity, Speed>,
    pub health: HashMap<Entity, Health>,
    pub shooters: HashMap<Entity, Shooter>,
    pub coin_invs: HashMap<Entity, CoinInv>,
}

impl World {
    pub fn new() -> Self {
        Self {
            round_num: 1,
            new_round: true,
            next_id: 2,
            player_entity: 1,
            game_objs: HashMap::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            speeds: HashMap::new(),
            health: HashMap::new(),
            shooters: HashMap::new(),
            coin_invs: HashMap::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.game_objs.remove(&entity);
        self.positions.remove(&entity);
        self.velocities.remove(&entity);
        self.speeds.remove(&entity);
        self.health.remove(&entity);
        self.shooters.remove(&entity);
        self.coin_invs.remove(&entity);
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

    pub fn with_speed(&mut self, entity: Entity, speed: Speed) -> &mut Self {
        self.speeds.insert(entity, speed);
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

    pub fn with_coins_inventory(&mut self, entity: Entity) -> &mut Self {
        self.coin_invs.insert(entity, CoinInv { collected: 0 });
        self
    }
}
