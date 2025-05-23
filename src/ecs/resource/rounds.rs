#[derive(Debug, PartialEq, Eq)]
pub enum RoundPhase {
    Warmup,
    Live,
    Ended,
}

pub struct RoundState {
    pub phase: RoundPhase,
    // pub time_remaining: f32,
    // pub current_round: u32,
    // pub freeze_time: bool,
}
