use ratatui::Frame;
use crate::{App, app::Screen};

pub mod main_screen;
pub mod second_screen;

pub fn render(frame: &mut Frame, app: &App) {
    
    match app.screen {
        Screen::Main => main_screen::render_main(frame, app),
        Screen::Counter => second_screen::render_counter(frame, app),
    }
}