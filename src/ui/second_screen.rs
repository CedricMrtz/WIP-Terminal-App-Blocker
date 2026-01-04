use ratatui::{
    Frame,
    widgets::{Block, Paragraph},
};
use crate::app::App;

pub fn render_counter( frame: &mut Frame, app: &App){
    let text = format!(
        "Counter Screen\n\nPressed: {}\n\n[Enter] Increment\n[1] Back",
        app.counter
    );

    frame.render_widget(
        Paragraph::new(text)
            .block(Block::bordered().title("Counter")),
        frame.area(),
    );
}