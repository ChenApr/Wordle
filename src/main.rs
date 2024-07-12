use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Write};
use console;

use serde_json::Value;

use clap::Parser;

use game::Game;


mod game;
use game::*;

mod args;
use args::*;

mod app;
use app::*;

mod words_gen;

mod builtin_words;

mod state;
mod config;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

//If the the parameter in arg are None, that in config will take over it.
fn ConfigOverride(arg: &mut Args, config: config::GameConfig) {
    if let Some(true) = config.random {
        arg.random = true;
    }
    if let Some(true) = config.difficult {
        arg.difficult = true;
    }
    if let Some(true) = config.stats {
        arg.stats = true;
    }
    if let None = arg.day {
        if let Some(d) = config.day {
            arg.day = Some(d);
        }
    }
    if let None = arg.seed {
        if let Some(s) = config.seed {
            arg.seed = Some(s);
        }
    }
    if let None = arg.final_set {
        if let Some(f) = config.final_set {
            arg.final_set = Some(f);
        }
    }
    if let None = arg.acceptable_set {
        if let Some(a) = config.acceptable_set {
            arg.acceptable_set = Some(a);
        }
    }
    if let None = arg.state {
        if let Some(s) = config.state {
            arg.state = Some(s);
        }
    }
    if let None = arg.word {
        if let Some(w) = config.word {
            arg.word = Some(w);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        clear_screen();
    }

    //Initializing an arg to receive args
    let mut args_game = args::Args::parse();

    //Exam on the existence of config file
    //Note: should be taken right after the initialization of args_game
    if let Some(c) = &args_game.config {


        let mut _config = config::load_config(c).expect("Config file damaged");
        
        ConfigOverride(&mut args_game, _config);
        
    }

    
    //to keep game in loop if Y
    let mut is_game_on = true;

    //state recorder
    let mut state_game: state::GameMaster = state::GameMaster_new();

    let mut word_bank: HashMap<String, i32> = HashMap::new();

    let mut win = 0;
    let mut total = 0;
    let mut round = 0;

    //If state file exist, update the state_game
    if let Some(f_name) = args_game.state.clone() {
        if let Ok(mut f) =  std::fs::File::open(&f_name) {
            let mut data = String::new();
            std::io::Read::read_to_string(&mut f, &mut data).unwrap();

            if data.trim() == "{}" || data.trim() == "" {
                //data = "{\n \"total_rounds\": 0,\n \"games\": []\n}".to_string();}

                //A more elegant writing style
                data = r#" 
                {
                 "total_rounds": 0,
                 "games": []
                }
                
                "#.to_string();
            }

            match serde_json::from_str(&data) {
                Ok(json) =>  {
                    state_game = json;
                    for _game in &state_game.games {
                        for word in _game.guesses.clone() {
                            *word_bank.entry(word).or_insert(0) += 1;
                            round += 1;
                        }
                        if _game.guesses.contains(&_game.answer) {
                            win += 1;
                        }
                        else {
                            round -= 6;
                        }
                        total += 1;
                    }
                }
                Err(_) => {
                    panic!("State file damaged!")
                }
            }
        }
    }


    let mut day_added: u64 = 0;

    //Entry of game
    while is_game_on {

        if is_tty {
            is_game_on = false;
        }

        if let None = args_game.word {}
        else {
            is_game_on = false;
        }
        if is_tty {
            println!("Welcome to {}", console::style("Wordle").bold().bright().green())
        }
            
            //Initializing a game
            let mut game = Game::new(args_game.random, 
                args_game.difficult, 
                Some(args_game.day.unwrap_or(1) + day_added), 
                args_game.seed, is_tty,
                args_game.final_set.clone(),
                args_game.acceptable_set.clone());
            
            match game.receive_answer(&args_game.word) {
                Ok(_) => {}
                Err(_) => panic!("Failed to specifying the answer. Not in wordlist.")
            }

            match game.game_on() {

                Ok((guesses, _round)) => {
                    for guess in guesses {
                        *word_bank.entry(guess).or_insert(0) += 1;
                    }
                    win += 1;
                    total += 1;
                    round += _round;
                }

                Err(Error::GameLose(guesses)) => {
                    for guess in guesses {
                        *word_bank.entry(guess).or_insert(0) += 1;
                    }
                    total += 1;
                }

                _ => unimplemented!("Game loop error!")
            }

            if args_game.stats {

                //Inspired from ChatGPT
                // 提取键值对并排序
                //let mut items: Vec<_> = map.iter().collect();
                //items.sort_by(|a, b| b.1.cmp(a.1)); // 根据值降序排序
                let mut word_bank_vec: Vec<(String, i32)> = word_bank.clone().into_iter().collect();
                
                //sort by descending order of alphabet and ascending order of value.
                word_bank_vec.sort_by(|a, b| a.0.cmp(&b.0));
                word_bank_vec.sort_by(|a, b| b.1.cmp(&a.1));

                let mut ratio = 0.00;
                if win != 0 {
                    ratio = round as f64 / win as f64;
                }
                
                //print X Y Z: win lose avg_round_in_win
                println!("{} {} {:.2}", win, total - win, ratio);

                //print five the most frequently occurring words
                let mut cnt = 0;
                for i in word_bank_vec {
                    if cnt >= 5 {
                        break;
                    }
                    if cnt == 0 {
                        print!("{} {}",i.0, i.1);
                    }
                    else {
                        print!(" {} {}",i.0, i.1);
                    }
                    cnt += 1;
                }
                io::stdout().flush().unwrap();
                println!("");
            }

            if is_tty{
                print!("Start one more game? [Y/N]: ");
                io::stdout().flush().unwrap();
            }

            let mut is_continue = String::new();
            io::stdin().read_line(&mut is_continue).expect("Reading Error");

            let is_continue = is_continue.trim().to_string();
            
            if is_continue == "Y".to_string() 
                || is_continue == "y".to_string()
                || is_continue == "Yes".to_string()
                || is_continue == "yes".to_string(){
                    is_game_on = true;
                }
            else if is_continue == "N".to_string() 
                || is_continue == "0".to_string() 
                || is_continue == "n".to_string()
                || is_continue == "No".to_string()
                || is_continue == "no".to_string() {
                is_game_on = false;
            }

            if let Some(f_name) = args_game.state.clone() {
                
                state_game.total_rounds += 1;
                state_game.games.push(state::Game { answer: game.answer.to_uppercase(), guesses: game.guesses});
                
                let json = serde_json::to_string_pretty(&state_game).unwrap();
                std::fs::write(f_name, json).unwrap();
            }

        

        day_added += 1;
        

    }

    if is_tty {
        clear_screen();
    }

   

}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Ok(Box::new(app::GuiApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}