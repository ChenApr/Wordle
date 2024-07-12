use crate::game;
use crate::LetterState;

use super::metrics;
use super::letter;


//rendering the guesses and answer
pub fn guesses_grid(ui: &mut egui::Ui, game: &game::Game, guess: &String, diff: bool) {
    for i in 0..metrics::ROWS {
        if i < game.round - 1 {
            for j in 0..metrics::COLUMNS {
                let _guess = &game.guesses[i as usize];
                let _char = _guess.chars().nth(j as usize).unwrap();
                let _state = game.guesses_state[i as usize][j as usize].clone();
                letter::letter(ui, letter::Letter {letter: Some(_char), state: _state, row: i.clone(), column: j.clone()}, false, diff);
            }
        }
        else if i == game.round - 1{
            let mut  pos = 0;
            for _char in guess.chars() {
                letter::letter(ui, letter::Letter{letter: Some(_char), state: game::LetterState::Unknown, row: i.clone(), column: pos.clone()}, false, diff);
                pos += 1;
            }
            while pos < metrics::COLUMNS {
                letter::letter(ui, letter::Letter{letter: Some(' '), state: game::LetterState::Unknown, row: i.clone(), column: pos.clone()}, false, diff);
                pos += 1;
            }
        }
        else {
            for j in 0..metrics::COLUMNS {
                letter::letter(ui, letter::Letter{letter: Some(' '), state: game::LetterState::Unknown, row: i.clone(), column: j.clone()}, false, diff);
            }
        }
        
    }
}

pub fn answer_grid (ui: &mut egui::Ui, answer: &String, answer_state: [LetterState; 5], diff: bool){
    let mut x = 0;
    for _char in answer.to_uppercase().chars() {
        letter::letter(ui, letter::Letter{letter: Some(_char), state: answer_state[x as usize], row: 6, column: x.clone()}, true, diff);
        x += 1;
    }
}