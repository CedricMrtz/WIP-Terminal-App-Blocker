use ratatui::Frame;
use crate::{App, app::Screen, app::Modal};

pub mod components;
pub mod main_screen;
pub mod configuration_screen;
pub mod edit_modal;
pub mod delete_modal;

pub fn render(frame: &mut Frame, app: &mut App) {
    
    match app.screen {
        Screen::Main => main_screen::render_main(frame, app),
        Screen::Configuration => configuration_screen::render_configuration(frame, app),
    }

    match app.selected_modal{
        Modal::EditRule => edit_modal::render_edit_modal(frame, app),
        Modal::DeleteRule => delete_modal::render_delete_modal(frame, app),
        _ => {}
    }
}