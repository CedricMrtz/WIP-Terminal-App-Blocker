use ratatui::{
    Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};
use crate::app::App;

pub fn render_main(frame: &mut Frame, app: &App){
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();
        let text = app.status_message.to_string();
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }