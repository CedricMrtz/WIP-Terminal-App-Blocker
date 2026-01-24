use ratatui::{
    Frame, layout::{Alignment, Rect}, style::{Color, Style},  widgets::{Block, Borders, Paragraph}
};
use crate::app::App;

pub fn render_rulesdescription(frame: &mut Frame, area: Rect, _app: &App){
        
        frame.render_widget(
            Paragraph::new("Info")
                .block(Block::new().borders(Borders::ALL).title("Description"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            area,
        );
    }