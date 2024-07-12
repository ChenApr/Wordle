use crate::{game, GameState, LetterState, KEYBOARD_1, KEYBOARD_2, KEYBOARD_3};
use egui::{Align2, Color32, FontFamily, InputState, Key, Modifiers, Pos2, Rect, Sense};
use super::utils;
use utils::*;
use super::metrics;
use metrics::*;

pub const ENTER: char = '\n';
pub const BACKSPACE: char = '\x08';

//Adapted from https://github.com/abmfy/wordle/tree/master from abmfy on 2024-07-10
//An elegant way to realize getting the keyinput and rendering at the same time by using clousure and call-back function
fn get_fill_color(state: &LetterState, diff: bool) -> Color32 {
    match state {
        LetterState::Green => MY_GREEN,
        LetterState::Red => if diff {MY_BLACK} else {MY_GRAY},
        LetterState::Yellow => MY_YELLOW,
        LetterState::Unknown => if diff {MY_GRAY} else {MY_GRAY_KEY},
    }
}

fn get_text_color(state: &LetterState, diff: bool) -> Color32 {
    match state {
        LetterState::Unknown => if diff {MY_WHITE} else {Color32::BLACK},
        _ => if diff {Color32::WHITE} else {Color32::WHITE},
    }
}

fn get_pos(_char: &char) -> (i32, i32) {
    if *_char == '\n' {
        return (2, 0)
    }

    if *_char == '\x08' {
        return (2, 8)
    }
    let mut x: i32 = 0;
    for i in KEYBOARD_1.chars() {
        if *_char == i {
            return (0, x)
        }
        x += 1;
    }
    x = 0;
    for i in KEYBOARD_2.chars() {
        if *_char == i {
            return (1, x)
        }
        x += 1;
    }
    x = 1;
    for i in KEYBOARD_3.chars() {
        if *_char == i {
            return (2, x)
        }
        x += 1;
    }


    (-1,-1)
}

fn key_grid(ui: &mut egui::Ui, _char: &char, _state: &LetterState, (_x, _y): &(i32, i32), diff: bool ) -> bool {
    let point = get_key_start_point(ui);

    let mut box_size = get_button_size(ui);

    let mut x = point.x + (*_y as f32 - 4.5) * box_size.x;
    let mut y = point.y + (*_x as f32 - 1.5) * box_size.y;

    if *_x == 0 {
        x = x - 0.5 * box_size.x;
    }

    if *_x == 2 {
        if *_y == 0 {
            x -= 0.5 * box_size.x;
            box_size.x = 1.6 * box_size.x;
        }
        if *_y == 8 {
            box_size.x = 1.5 * box_size.x;
        }

    }

    let rect: Rect = Rect::from_min_size(
        Pos2 {y, x},
        box_size * 0.9
    );

    let mut response = ui.allocate_rect(rect, Sense::click());

    

    let _fill_color = get_fill_color(_state, diff);
    let _text_color = get_text_color(_state, diff);

    if(*_y == 0 && *_x == 2) {
        ui.painter().rect(rect, 4.0, Color32::GRAY, (0.0, Color32::WHITE ));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, "ENTER", egui::FontId { size: 0.27 * box_size.x, family: FontFamily::Name("SF".into()) }, _text_color);
    }
    else if(*_y == 8 && *_x == 2) {
        ui.painter().rect(rect, 4.0, Color32::GRAY, (0.0, Color32::WHITE ));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, "BACK", egui::FontId { size: 0.3 * box_size.x, family: FontFamily::Name("SF".into()) }, _text_color);
    }
    else {
        ui.painter().rect(rect, 4.0, _fill_color, (0.0, Color32::WHITE ));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, _char.to_uppercase(), egui::FontId { size: 0.8 * box_size.x, family: FontFamily::Name("SF".into()) }, _text_color);
    }
    response.clicked()
}

pub fn keyboard(ui: &mut egui::Ui, game: &game::Game, diff: bool) -> Option<char> {
    let mut press:Option<char> = None;

    const KEYS: [egui::Key; 26] = [
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
    ];
    
    for i in KEYBOARD_1.chars() {
        if key_grid(ui, &i, game.Letters.get(&i).as_ref().unwrap(), &get_pos(&i), diff) {
            press = Some(i.to_uppercase().next().unwrap());
        }
    }
    for i in KEYBOARD_2.chars() {
        if key_grid(ui, &i, game.Letters.get(&i).as_ref().unwrap(), &get_pos(&i), diff) {
            press = Some(i.to_uppercase().next().unwrap());
        }
    }
    for i in KEYBOARD_3.chars() {
        if key_grid(ui, &i, game.Letters.get(&i).as_ref().unwrap(), &get_pos(&i), diff) {
            press = Some(i.to_uppercase().next().unwrap());
        }
    }
    if key_grid(ui, &'\n', &LetterState::Unknown, &get_pos(&'\n'), diff) {
        press = Some('\n');
    }
    if key_grid(ui, &'\x08', &LetterState::Unknown, &get_pos(&'\x08'), diff) {
        press = Some('\x08');
    }

    ui.input_mut(|input: &mut InputState| {
        if input.consume_key(Modifiers::NONE, Key::Enter) {
            press = Some('\n');
        }
        else if input.consume_key(Modifiers::NONE, Key::Backspace) {
            press = Some('\x08');
        }
        else if input.consume_key(Modifiers::SHIFT, Key::Enter) {
            press = Some('\n');
        }
        else if input.consume_key(Modifiers::SHIFT, Key::Backspace) {
            press = Some('\x08');
        }
        else {
            for key in KEYS {
                if input.consume_key(Modifiers::NONE, key) {
                    press = format!("{key:?}").chars().nth(0);
                    break;
                }
                if input.consume_key(Modifiers::SHIFT, key) {
                    press = format!("{key:?}").chars().nth(0);
                    break;
                }
            }
        }
    });
    
    press
}