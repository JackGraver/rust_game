use macroquad::color::Color;

#[derive(PartialEq, Eq)]
pub enum GameObjectType {
    Player,
    Bullet,
    Enemy,
    Coin,
}

pub struct GameObject {
    pub obj_type: GameObjectType,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
}

#[derive(Clone, Copy)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

pub struct Speed(pub f32);

#[derive(Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

pub struct Enemy;

pub struct Bullet;

pub struct Shooter {
    pub cooldown: f32,
}

pub struct CoinInv {
    pub collected: i32,
}
