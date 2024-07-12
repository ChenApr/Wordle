use egui::{Color32, Id, Rect, Pos2, Vec2};

//Measuring tools for renderingS
pub fn get_screen_rect(ui: &egui::Ui) -> egui::Rect {
    ui.ctx().screen_rect()
}

pub fn get_start_point(ui: &egui::Ui) -> Pos2 {

    let rect = get_screen_rect(ui);

    let x = rect.min.x + 0.5 * rect.size().x;
    let y = rect.min.y + 0.37 * rect.size().y;
    Pos2 {x, y}

}

pub fn get_key_start_point(ui: &egui::Ui) -> Pos2 {

    let rect = get_screen_rect(ui);

    let x = rect.min.x + 0.5 * rect.size().x;
    let y = rect.min.y + 0.87 * rect.size().y;
    Pos2 {x, y}

}

pub fn get_box_size(ui: &egui::Ui) -> Vec2 {

    let rect = get_screen_rect(ui);

    let x = rect.size().y * 0.075;
    let y = x;

    Vec2 {x, y}
}

pub fn get_button_size(ui: &egui::Ui) -> Vec2 {
    
    let rect = get_screen_rect(ui);

    let y = rect.size().y * 0.07;
    let x = y * 0.7;

    Vec2 {x, y}
}