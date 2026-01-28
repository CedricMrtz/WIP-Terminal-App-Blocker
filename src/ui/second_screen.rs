use ratatui::{
    Frame, layout::Alignment, widgets::{Block, Paragraph}
};
use crate::app::App;

pub fn render_counter( frame: &mut Frame, _app: &App){

    frame.render_widget(
        Paragraph::new("AAA")
            .block(Block::bordered()
            .title("Configuration")
            .title_alignment(Alignment::Center)),
        frame.area(),
    );
}