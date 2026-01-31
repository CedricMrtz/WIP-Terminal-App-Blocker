use ratatui::Frame;
use crate::{App, app::Screen, app::Modal};

pub mod components;
pub mod main_screen;
pub mod configuration_screen;
pub mod edit_modal;

pub fn render(frame: &mut Frame, app: &mut App) {
    
    match (app.screen, app.selected_modal) {
        (Screen::Main, Modal::None) => main_screen::render_main(frame, app),
        (Screen::Configuration, Modal::None) => configuration_screen::render_configuration(frame, app),
        (Screen::Main, Modal::EditRule) => edit_modal::render_edit_modal(frame, app),
        _ => {}
    }
}