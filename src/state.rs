use serde_json::{Result, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GameMaster {
    pub total_rounds: i32,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub answer: String,
    pub guesses: Vec<String>,
}

pub fn GameMaster_new() -> GameMaster {
    GameMaster {total_rounds: 0, games: Vec::new()}
}

pub fn Game_new() -> Game {
    Game {answer: String::new(), guesses: Vec::new()}
}