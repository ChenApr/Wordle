use crate::state;
use chrono::{self, DateTime, Utc};
use serde::{Deserialize, Serialize};


// A game history recorder
#[derive(Deserialize, Serialize)]
pub struct Game {
    pub game: state::Game,
    pub seed: u64,
    pub day: u64,
    pub date: DateTime<Utc>,
    pub win: bool,
    pub round: i32,
}