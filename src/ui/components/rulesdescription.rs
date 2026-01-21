use ratatui::{
    Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, symbols::block, text::{Line, Span}, widgets::{Block, Borders, Paragraph}
};
use crate::app::App;

pub fn render_rulesdescription(frame: &mut Frame, area: Rect, app: &App){
        
        frame.render_widget(
            Paragraph::new("Info")
                .block(Block::new().borders(Borders::ALL).title("Description"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            area,
        );
    }