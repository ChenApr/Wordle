
use std::{collections::HashMap, hash::Hash, io::SeekFrom};
use clap::builder::Str;
use eframe::Frame;
use egui::epaint::HAS_RAYON;
use egui::{Color32, DragValue};
use egui::{FontData, FontDefinitions, FontFamily, RichText, Window, SidePanel};
use guesses::{answer_grid, guesses_grid};
use metrics::{MY_GREEN, MY_RED};
use web_sys::console;   

mod guesses;
mod metrics;
mod letter;
mod utils;
mod keyboard;
mod gamemaster;

use crate::builtin_words;
use crate::game::Game;
use crate::words_gen;
use crate::GameState;
use words_gen::*;
use crate::game;
use game::LetterState;
use crate::state::{self, GameMaster, GameMaster_new};
use crate::config;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GuiApp {
    game: Option<game::Game>,
    guess: String,
    win: i32,
    lose: i32,
    round: i32,
    day: u64,
    seed: u64,
    difficult: bool,
    word_used: HashMap<String, i32>,
    word_used_vec: Vec<(String, i32)>,
    game_history: Vec<gamemaster::Game>,
    config: config::GameConfig,
}

impl Default for GuiApp {
    fn default() -> Self {
        Self {
            game: None,
            guess: String::new(),
            win: 0,
            lose: 0,
            round: 0,
            day: 1,
            seed: 2024,
            difficult: false,
            word_used: HashMap::new(),
            word_used_vec: Vec::new(),
            game_history: Vec::new(),
            config: config::GameConfig::new()
        }
    }
}

impl GuiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert("NY".to_string(), FontData::from_static(include_bytes!("../assets/NewYorkExtraLarge-Bold.otf")));
        fonts.families.insert(FontFamily::Name("NY".into()), vec!["NY".to_string()]);
        
        fonts.font_data.insert("SF".to_string(), FontData::from_static(include_bytes!("../assets/SF-Pro-Display-Bold.otf")));
        fonts.families.insert(FontFamily::Name("SF".into()), vec!["SF".to_string()]);
        
        fonts.font_data.insert("SFM".to_string(), FontData::from_static(include_bytes!("../assets/SF-Mono-Medium.otf")));
        fonts.families.insert(FontFamily::Name("SFM".into()), vec!["SFM".to_string()]);

        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "SFM".to_string());

        cc.egui_ctx.set_fonts(fonts);

        let mut app: GuiApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        }
        else {
            GuiApp::default()
        };

        if let None = app.game {
            app.renew_game(config::GameConfig::new_with_day(1));
        }
        app
    }

    fn renew_game(&mut self, config: config::GameConfig){
        let mut game = Game::new(config.random.unwrap_or(true),
        config.difficult.unwrap_or(false),
        config.day,
        config.seed, 
        false, 
        config.final_set, 
        config.acceptable_set);

        game.receive_answer(&config.word);
        
        self.game = Some(game);
    }
}

impl eframe::App for GuiApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        

        //adding background color to difficult mode
        if self.difficult {
            let mut visuals = egui::Visuals::dark(); 
            visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255)); 
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 30); 
            ctx.set_visuals(visuals);
        }
        else {
            let mut visuals = egui::Visuals::light(); 
            visuals.override_text_color = Some(egui::Color32::from_rgb(0, 0, 0)); 
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(225, 225, 225); 
            ctx.set_visuals(visuals);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(RichText::new("Wordle")
                    .family(FontFamily::Name("NY".into()))
                    .size(72.0)
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            
            egui::ScrollArea::vertical().show(ui, |ui|{
            ui.vertical(|ui| {

            ui.label(RichText::new("Wordle").family(FontFamily::Name("NY".into())).size(24.0).color(Color32::GOLD));
            ui.label(RichText::new("- Guess a word within 6 tries!").family(FontFamily::Name("SFM".into())).size(14.0).italics());
            ui.label(RichText::new("- Press enter for a new game").family(FontFamily::Name("SFM".into())).size(14.0).italics().color(Color32::PLACEHOLDER));
            ui.label(RichText::new("  every time you win or lose.").family(FontFamily::Name("SFM".into())).size(14.0).italics().color(Color32::PLACEHOLDER));


            egui::CollapsingHeader::new(RichText::new("Settings").family(FontFamily::Name("SFM".into())).size(20.0)).show(ui, |ui| {
                
                ui.checkbox(&mut self.difficult, RichText::new("Difficult").family(FontFamily::Name("SFM".into())).size(14.0));

                if self.difficult {
                    self.config.difficult = Some(true);
                    self.game.as_mut().unwrap().difficult = true;
                }
                else {
                    self.config.difficult = None;
                    self.game.as_mut().unwrap().difficult = false;
                }

                

                ui.label(RichText::new("Seed").family(FontFamily::Name("SFM".into())).size(14.0));
                
                ui.add(DragValue::new(&mut self.seed));

                
                ui.label(RichText::new("Day").family(FontFamily::Name("SFM".into())).size(14.0));
                
                ui.add(DragValue::new(&mut self.day));

                ui.label("Config");
                if ui.button("Enter").clicked() {
                    self.config.seed = Some(self.seed);
                    self.config.day = Some(self.day);
                    self.guess.clear();
                    self.renew_game(self.config.clone());
                }
            });
            egui::CollapsingHeader::new(RichText::new("Statistics").family(FontFamily::Name("SFM".into())).size(20.0)).show(ui, |ui| {
                
                ui.label(RichText::new(format!("Win: {}", self.win)).family(FontFamily::Name("SFM".into())).size(14.0).color(metrics::MY_GREEN));
                ui.label(RichText::new(format!("Fail: {}", self.lose)).family(FontFamily::Name("SFM".into())).size(14.0).color(metrics::MY_RED));
                ui.label(RichText::new(format!("Average Rounds: {}", self.round as f64 / self.win as f64)).family(FontFamily::Name("SFM".into())).size(14.0));

                ui.label("\n");
                ui.label(RichText::new("Most used words:").family(FontFamily::Name("SFM".into())).size(14.0));
                
                let mut cnt = 0;
                for i in &self.word_used_vec {
                    if cnt >= 5 {
                        break;
                    }
                    ui.label(RichText::new(format!("{}: {} times", i.0, i.1)).family(FontFamily::Name("SFM".into())).size(14.0));
                    cnt += 1;
                }

                if ui.button("Reset").clicked() {
                    self.win = 0;
                    self.lose = 0;
                    self.round = 0;
                    self.word_used = HashMap::new();
                    self.word_used_vec = Vec::new();
                    
                }
        });

            egui::CollapsingHeader::new(RichText::new("GameHistory").family(FontFamily::Name("SFM".into())).size(20.0)).show(ui, |ui| {
                
                for i in &self.game_history {
                    egui::CollapsingHeader::new(RichText::new(format!("{}", i.date)).family(FontFamily::Name("SFM".into())).size(14.0)).show(ui, |ui| {
                        
                        ui.label(RichText::new(format!("seed: {}, day: {}", i.seed, i.day)).family(FontFamily::Name("SFM".into())).size(14.0));
                        ui.label(RichText::new(format!("answer:")).family(FontFamily::Name("SFM".into())).size(14.0));
                        ui.label(RichText::new(format!("{}", i.game.answer.to_uppercase())).family(FontFamily::Name("SFM".into())).size(14.0).color(MY_GREEN));
                        ui.label(RichText::new(format!("guesses:")).family(FontFamily::Name("SFM".into())).size(14.0));
                        for j in 0..i.round - 1 {
                            ui.label(RichText::new(format!("{}", i.game.guesses[j as usize])).family(FontFamily::Name("SFM".into())).size(14.0).color(MY_RED));
                        }
                        if i.win {
                            ui.label(RichText::new(format!("{}", i.game.guesses[(i.round - 1) as usize])).family(FontFamily::Name("SFM".into())).size(14.0).color(MY_GREEN));
                        }
                        else {
                            ui.label(RichText::new(format!("{}", i.game.guesses[(i.round - 1) as usize])).family(FontFamily::Name("SFM".into())).size(14.0).color(MY_RED));
                        }
                });
                    
                }

                if ui.button("Clear").clicked() {
                    
                    self.game_history = Vec::new();
                }
        });

        
    });
});

            guesses_grid(ui, self.game.as_ref().unwrap() , &self.guess, self.difficult);

            if let game::GameState::Lose = self.game.as_ref().unwrap().game_state {
                answer_grid(ui, &self.game.as_ref().unwrap().answer, self.game.as_ref().unwrap().answer_state, self.difficult);
            }

            if let Some(key) = keyboard::keyboard(ui, self.game.as_ref().unwrap(), self.difficult) {
                match key {
                    '\n' => {
                        if let game::GameState::FullString = self.game.as_ref().unwrap().game_state {
                            match self.game.as_ref().unwrap().check(&self.guess.to_lowercase()) {
                                Ok(()) => {
                                    console::log_1(&"Arrive here".into());
                                    self.game.as_mut().unwrap().round += 1;
                                    self.game.as_mut().unwrap().letters_update(&self.guess);

                                    self.game_history.push( gamemaster::Game { 
                                        game: state::Game { 
                                            answer: self.game.as_ref().unwrap().answer.clone(), 
                                            guesses: self.game.as_ref().unwrap().guesses.clone()
                                         }, 
                                        seed: self.seed.clone(), 
                                        day: self.day.clone(), 
                                        date: chrono::Utc::now(),
                                        win: true,
                                        round: self.game.as_ref().unwrap().round - 1});

                                    *self.word_used.entry(self.guess.clone()).or_insert(0) += 1;
                                    
                                    self.word_used_vec = self.word_used.clone().into_iter().collect();
                                    
                                    self.word_used_vec.sort_by(|a, b| a.0.cmp(&b.0));
                                    self.word_used_vec.sort_by(|a, b| b.1.cmp(&a.1));

                                    self.guess.clear();
                                    self.game.as_mut().unwrap().game_state = GameState::Win;
                                    self.win += 1;
                                    self.round += 1;
                                    console::log_1(&"Out here".into());
                                    
                                }
                                Err(game::Error::WrongAnswer) => {
                                    console::log_1(&"Arrive Wrongly here".into());
                                    self.game.as_mut().unwrap().round += 1;
                                    
                                    self.game.as_mut().unwrap().letters_update(&self.guess);


                                    *self.word_used.entry(self.guess.clone()).or_insert(0) += 1;

                                    self.word_used_vec = self.word_used.clone().into_iter().collect();
                                    
                                    self.word_used_vec.sort_by(|a, b| a.0.cmp(&b.0));
                                    self.word_used_vec.sort_by(|a, b| b.1.cmp(&a.1));
                                    
                                    self.game.as_mut().unwrap().game_state = GameState::Going;
                                    
                                    if self.game.as_ref().unwrap().round == 7 {
                                        self.game.as_mut().unwrap().game_state = GameState::Lose;
                                        self.lose += 1;
                                        self.round += 1;

                                        self.game_history.push( gamemaster::Game { 
                                            game: state::Game { 
                                                answer: self.game.as_ref().unwrap().answer.clone(), 
                                                guesses: self.game.as_ref().unwrap().guesses.clone()
                                             }, 
                                            seed: self.seed.clone(), 
                                            day: self.day.clone(), 
                                            date: chrono::Utc::now(),
                                            win: false,
                                            round: self.game.as_ref().unwrap().round - 1 });
                                    }

                                    self.guess.clear();

                                }
                                Err(game::Error::DisobeyingDifficult) => {
                                    console::log_1(&"Hint not used".into());
                                    console::log_1(&self.guess.clone().into());
                                }
                                _ => {}
                            }
                        }
                        else if let game::GameState::Going = self.game.as_ref().unwrap().game_state {}
                        else {
                            let mut _day = self.config.day.clone().unwrap_or(1);
                            _day += 1;
                            self.day += 1;
                            self.config.day = Some(_day);
                            self.renew_game(self.config.clone());
                            ctx.request_repaint();
                        }
                    }
                    '\x08' => {
                        if let game::GameState::Going = self.game.as_ref().unwrap().game_state {
                            if !self.guess.is_empty() {
                                self.guess.pop();
                            }
                            return
                        }
                        else if let game::GameState::FullString = self.game.as_ref().unwrap().game_state {
                            self.guess.pop();
                            self.game.as_mut().unwrap().game_state = GameState::Going;
                        }
                    }
                    c => {
                        if let game::GameState::Going = self.game.as_ref().unwrap().game_state {
                            self.guess.push(c);
                            if self.guess.len() == 5 {
                                self.game.as_mut().unwrap().game_state = GameState::FullString;
                                console::log_1(&"String fulled".into());
                            }
                        }
                    }
                }
            }
        });
    }
}
