use ratatui::{
    Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, symbols::block, text::{Line, Span}, widgets::{Block, Borders, Paragraph}
};
use crate::app::App;

pub fn render_ruleslist(frame: &mut Frame, area: Rect, app: &App){
        
        frame.render_widget(
            Paragraph::new(app.status_message.as_str())
                .block(Block::new().borders(Borders::ALL).title("Rules"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Yellow)),
            area,
        );
    }