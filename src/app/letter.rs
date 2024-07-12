
use egui::{Align2, Color32, FontFamily, Pos2, Rect, Vec2, Sense};

use super::utils;
use utils::*;

use super::metrics;
use metrics::*;

use crate::game::LetterState;

pub struct Letter {
    pub letter: Option<char>,
    pub state: LetterState,
    pub row: i32,
    pub column: i32,
}

impl Letter {
    fn get_fill_color(&self, diff: bool) -> Color32 {
        
        match self.state {
            LetterState::Green => MY_GREEN,
            LetterState::Red => MY_GRAY,
            LetterState::Yellow => MY_YELLOW,
            LetterState::Unknown => if diff {Color32::from_rgb(27, 27, 27)} else {MY_WHITE},
        }
    }

    fn get_stroke_color(&self, dark: bool, diff: bool) -> Color32 {
        if dark {
            if diff {
                return MY_GRAY
            }
            else {
                return MY_WHITE
            }
        }
        match self.state {
            LetterState::Green => MY_GREEN,
            LetterState::Red => MY_GRAY,
            LetterState::Yellow => MY_YELLOW,
            LetterState::Unknown => if diff {MY_BLACK} else {MY_GRAY},
        }
    }

    fn get_text_color(&self, dark: bool, diff: bool) -> Color32 {
        if dark {
            if diff {
                return Color32::WHITE
            }
            else {
                return Color32::BLACK
            }
        }
        match self.state {
            LetterState::Unknown => if diff {Color32::WHITE} else {Color32::BLACK},
            _ => Color32::WHITE,
        }
    }
}

//Adapted from https://github.com/abmfy/wordle/tree/master from abmfy on 2024-07-10
//An elegant way to realize getting the keyinput and rendering at the same time by using clousure and call-back function
//The measuring part was simplified, while the idea of pixels differences in different devices is important.
pub fn letter(ui: &mut egui::Ui, _letter: Letter, dark: bool, diff: bool) {
    
    let point = get_start_point(ui);

    let box_size = get_box_size(ui);

    let x = point.x + (_letter.column as f32 - 2.5) * box_size.x;
    let mut y = point.y + (_letter.row as f32 - 2.5) * box_size.y;

    if dark {
        y = y + 0.35 * box_size.y;
    }

    let rect: Rect = Rect::from_min_size(
        Pos2 {x, y},
        box_size * 0.9
    );

    ui.allocate_rect(rect, Sense::hover());

    let _fill_color = _letter.get_fill_color(diff);
    let _stroke_color = _letter.get_stroke_color(dark, diff);
    let _text_color = _letter.get_text_color(dark, diff);

    ui.painter().rect(rect, 0.0, _fill_color, (2.0, _stroke_color ));
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, _letter.letter.unwrap_or(' '), egui::FontId { size: 0.8 * box_size.x, family: FontFamily::Name("SF".into()) }, _text_color);

}