use std::{collections::HashMap, hash::Hash, io::SeekFrom};
use std::io::{self, Write};
use console::{self, user_attended};
use serde::{Deserialize, Serialize};


use crate::builtin_words::*;
use serde_json::json;

use crate::words_gen;
use words_gen::*;


pub const MAX_GUESS_TIMES: i32 = 6;
pub const ALPHABET : &str = "abcdefghijklmnopqrstuvwxyz";
pub const KEYBOARD_1: &str = "qwertyuiop";
pub const KEYBOARD_2: &str = "asdfghjkl";
pub const KEYBOARD_3: &str = "zxcvbnm";

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum LetterState {
    Green,
    Red,
    Yellow,
    Unknown,
}

pub enum Error {
    NotInWordList,
    WrongAnswer,
    DisobeyingDifficult,
    GameLose(Vec<String>)
}

#[derive(Deserialize, Serialize)]
pub enum GameState {
    FullString,
    Going,
    Win,
    Lose,
}

impl LetterState {
    fn to_state(&self) -> char {
        match self {
            Self::Green => 'G',
            Self::Red => 'R',
            Self::Yellow => 'Y',
            Self::Unknown => 'X',
            _ => unimplemented!("Not the recording state")
        }
    }
}

fn color_print(c: char, color: &LetterState) {
    match color {
        LetterState::Green => print!("{}",console::style(c.to_uppercase()).green()),

        LetterState::Red => print!("{}",console::style(c.to_uppercase()).red()),

        LetterState::Yellow => print!("{}",console::style(c.to_uppercase()).yellow()),
        
        LetterState::Unknown => print!("{}",console::style(c.to_uppercase())),
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

//Adapted from https://github.com/abmfy/wordle/tree/master from abmfy on 2024-07-07
//A referrence to the structure and struct relationships
#[derive(Deserialize, Serialize)]
pub struct Game {
    pub answer: String,
    pub answer_state: [LetterState; 5],
    pub guesses: Vec<String>,
    pub guesses_state: Vec<[LetterState; 5]>,
    pub Letters: HashMap<char, LetterState>,
    is_tty: bool,
    pub difficult: bool,
    random: bool,
    pub day: u64,
    pub seed: u64,
    word_bank: Words, 
    pub round: i32,
    pub game_state: GameState,
}

impl Game {

    //Initializing an object of Game
    pub fn new(_random: bool, 
        _difficult: bool,
        _day: Option<u64>,
        _seed: Option<u64>,
        _is_tty: bool,
        _f_set: Option<String>,
        _a_set: Option<String>) -> Game {

        let mut _letters: HashMap<char, LetterState> = HashMap::new();
        for i in ALPHABET.chars() {
            _letters.insert(i, LetterState::Unknown);
        }

        let mut _words = Words::new();
        if let Some(f) = _f_set {
            match _words.FINAL_set(f) {
                Ok(_) => {}
                Err(words_gen::Error::DuplicatedWords) => panic!("Duplicated words in the final-set provided"),
                Err(words_gen::Error::BeyondBuiltinWords) => panic!("Final-set provided reach beyond the original set"),
                Err(words_gen::Error::FileNotExist) => panic!("File does not exist"),
            }
        }
        if let Some(a) = _a_set {
            match _words.ACCEPTABLE_set(a) {
                Ok(_) => {}
                Err(words_gen::Error::DuplicatedWords) => panic!("Duplicated words in the final-set provided"),
                Err(words_gen::Error::BeyondBuiltinWords) => panic!("Final-set provided reach beyond the original set"),
                Err(words_gen::Error::FileNotExist) => panic!("File does not exist"),
            }
        }

        let mut _day = _day;
        let mut _seed = _seed;


        Game {answer: String::new(), 
            answer_state:[LetterState::Unknown; 5], 
            guesses: Vec::<String>::new(), 
            guesses_state: Vec::new(),
            Letters: _letters, 
            is_tty: _is_tty, 
            difficult: _difficult, 
            random: _random,
            day: _day.unwrap_or(1),
            seed: _seed.unwrap_or(2024),
            word_bank: _words,
            round: 1,
            game_state: GameState::Going}
    }

    //Exam if the args.word is passed into game
    //if not and not in random, get an answer from stdout.
    pub fn receive_answer(&mut self, word: &Option<String>) -> Result<(),Error>{

        if self.random {
            self.word_bank.FINAL_RAND(self.seed);
            self.answer = self.word_bank.FINAL[(self.day - 1) as usize].clone();
            Ok(())
        }
        else {
            if let Some(_answer) = word {
                self.answer = _answer.to_lowercase().clone();
            }
            else{
                let mut answer: String = String::new();
                //this place has a fallback of input nothing.
                if self.is_tty {
                    print!("Please specify a {} {}: ",
                            console::style("five-letter").blue().bold(), 
                            console::style("answer").red().bold());
                    loop {
                        io::stdout().flush().unwrap();
                        answer = String::new();
                        io::stdin().read_line(&mut answer).expect("Wrong input");
                        answer = answer.trim().to_string();

                        if !self.word_bank.FINAL.contains(&answer.to_lowercase()) {
                            print!("{}, please specify again: ", console::style("Answer not in the word bank FINAL").red());
                            io::stdout().flush().unwrap();
                            continue;
                        }
                        else {
                            break;
                        }
                    }
                }
                else {
                    io::stdin().read_line(&mut answer).expect("Wrong input");
                    answer = answer.trim().to_string();
                }
                self.answer = answer.to_lowercase().clone();
            }

            if !self.word_bank.FINAL.contains(&self.answer.to_lowercase()) {
                Err(Error::NotInWordList)
            }
            else {
                Ok(())
            }
        }
    }

    //Entry of each game.
    pub fn game_on(&mut self) -> Result<(Vec<String>, i32), Error>{

        let mut round = 1;

        if self.is_tty{
            self.print_all();
        }

        while round <= 6 {

            if self.is_tty {
                println!("{} : {} / 6", console::style("Round").green(), console::style(round).blue());
                print!("Make a guess: ");
                io::stdout().flush().unwrap();
            }

            let mut guess: String = String::new();
            io::stdin().read_line(&mut guess).expect("Wrong input");
            let guess = guess.trim().to_string();

            match self.check(&guess.to_lowercase()) {

                Ok(()) => {
                    let _word_display = self.letters_update(&guess);

                    if !self.is_tty {
                        self.print_state(&_word_display);
                    }
                    
                    if self.is_tty {
                        self.print_all();
                        println!("{} {} {}", console::style("CORRECT").yellow(), console::style(self.answer.to_uppercase()).green(), round);
                    }
                    else {
                        println!("CORRECT {}", round);
                    }
                    
                    return Ok((self.guesses.clone(),round));
                }

                Err(Error::WrongAnswer) => {
                    let _word_display = self.letters_update(&guess);
                    if self.is_tty {
                        self.print_all();
                    }
                    else {
                        self.print_state(&_word_display);
                    }
                    round += 1;
                }

                Err(Error::NotInWordList) => {
                    if self.is_tty {
                        println!("{}",console::style("The word is not in word list, try once more.").red());
                        io::stdout().flush().unwrap();
                    }
                    else {
                        println!("INVALID");
                    }
                    continue;
                }

                Err(Error::DisobeyingDifficult) => {
                    if self.is_tty {
                        println!("{}",console::style("Not using the hint, try once more").red());
                        io::stdout().flush().unwrap();
                    }
                    else {
                        println!("INVALID");
                    }
                    continue;
                }

                _ => unimplemented!("Unknown mistakes")
            }
        }
        
        if round == 7 {
            if self.is_tty {
                print!("{} ", console::style("FAILED").red());
                let mut pos = 0;
                for i in self.answer.chars() {
                    color_print(i, &self.answer_state[pos]);
                    pos += 1;
                }
                io::stdout().flush().unwrap();
                println!("");
            }
            else {
                println!("FAILED {}", self.answer.to_uppercase());
            }
        }
        return Err(Error::GameLose(self.guesses.clone()));
    }

    //Print the word and current states of letters
    pub fn print_state(&self, word: &[LetterState; 5]) {
        for i in 0..5{
            print!("{}", word[i].to_state());
        }
        print!(" ");
        for i in ALPHABET.chars() {
            print!("{}", self.Letters.get(&i).unwrap().to_state());
        }
        println!("");
    }

    //Check whether the word is not in wordlist, correct or wrong answer.
    //If difficult mode is on, extra check will be carried out.
    pub fn check(&self, word: &String) -> Result<(), Error> {
        if !self.word_bank.ACCEPTABLE.contains(&word.to_lowercase()) {
            Err(Error::NotInWordList)
        }
        else {
            //get the right answer.
            if *word.to_lowercase() == self.answer {
                Ok(())
            }
            else {
                if self.difficult {
                    let mut pos = 0;
                    for i in self.answer_state {
                        if let LetterState::Green = i {
                            if word.chars().nth(pos).unwrap() != self.answer.chars().nth(pos).unwrap() {
                                return Err(Error::DisobeyingDifficult);
                            }
                        }
                        else if let LetterState::Yellow = i {
                            if !word.contains(self.answer.chars().nth(pos).unwrap()) {
                                return Err(Error::DisobeyingDifficult);
                            }
                        }
                        pos += 1;
                    }
                }
                Err(Error::WrongAnswer)
            }
        }
    }

    //Updating the 26 letters' states
    pub fn letters_update(&mut self, word: &String) -> [LetterState; 5] {

        let word = word.to_lowercase();

        //Build a counter of each letter in the answer
        let mut word_hash: HashMap<char, i32> = HashMap::new();
        //Build a state recorder of the passed-in word
        let mut _word_display = [LetterState::Unknown; 5];

        let mut pos = 0;
        
        //count the letter in answer.
        for i in self.answer.chars() {
            *word_hash.entry(i).or_insert(0) += 1;
        }

        //first round to find the green letter
        for i in word.chars() {
            if i == self.answer.chars().nth(pos).unwrap() {
                //Green should be first noted and no longer changed
                _word_display[pos] = LetterState::Green;
                self.Letters.insert(i, LetterState::Green);
                self.answer_state[pos] = LetterState::Green;
                *word_hash.entry(i).or_insert(1) -= 1;
            }
            pos += 1;
        }
        
        pos = 0;
        
        //second round to find the yellow letter
        for i in word.chars() {
            //no more occurrences, marked red.
            if *word_hash.entry(i).or_insert(0) == 0 {
                if let LetterState::Green = _word_display[pos] {}
                else {
                    _word_display[pos] = LetterState::Red;
                }
            }
            else {
                if let LetterState::Green = _word_display[pos]{}
                else {
                    _word_display[pos] = LetterState::Yellow;
                    for j in 0..5 {
                        if self.answer.chars().nth(j).unwrap() == word.chars().nth(pos).unwrap() {
                            if let LetterState::Green = self.answer_state[j] {}
                            else {
                                self.answer_state[j] = LetterState::Yellow;
                                break;
                            }
                        }
                    }
                    *word_hash.entry(i).or_insert(1) -= 1;
                }
                
            }
            pos += 1;
        }

        pos = 0;

        //update all letters based on the _word_display.
        for i in word.chars() {
            if let LetterState::Yellow = _word_display[pos]{
                if let LetterState::Green = self.Letters.get(&i).unwrap() {}
                else {
                    self.Letters.insert(i, LetterState::Yellow);
                }
            }
            else if let LetterState::Red = _word_display[pos] {
                if let LetterState::Unknown = self.Letters.get(&i).unwrap() {
                    self.Letters.insert(i, LetterState::Red);
                }
            }
            pos += 1;
        }
        
        self.guesses.push(word.to_uppercase().clone());
        self.guesses_state.push(_word_display.clone());
        _word_display
    }

    fn print_keyboard(&self) {
        for i in KEYBOARD_1.chars() {
            color_print(i, self.Letters.get(&i).unwrap());
        }
        io::stdout().flush().unwrap();
        println!("");

        for i in KEYBOARD_2.chars() {
            color_print(i, self.Letters.get(&i).unwrap());
        }
        io::stdout().flush().unwrap();
        println!("");

        for i in KEYBOARD_3.chars() {
            color_print(i, self.Letters.get(&i).unwrap());
        }
        io::stdout().flush().unwrap();
        println!("");
    }

    fn print_guesses(&self) {
        let mut x = 0;
        for i in &self.guesses {
            let mut y = 0;
            for j in i.chars() {
                color_print(j, &self.guesses_state[x][y]);
                y += 1;
            }
            println!("");
            x += 1;
        }
    }

    fn print_all(&self) {
        clear_screen();

        println!("{}", console::style("Wordle").bold().bright().green());

        println!("");

        self.print_guesses();

        println!("");

        self.print_keyboard();

        println!("");
    }


}