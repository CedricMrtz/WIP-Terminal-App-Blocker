use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph}
};
use crate::app::App;

pub fn render_rulesdescription(frame: &mut Frame, area: Rect, app: &App){
        
        let selected_rule = app
            .selection
            .selected()
            .and_then(|i| app.rules.get(i));

        let description = Paragraph::new(
            selected_rule
            .and_then(|r| r.description.as_ref())
            .map(|d| d.as_str())
            .unwrap_or("No description available.")
        );
        
        frame.render_widget(
            description
                .block(Block::new().borders(Borders::ALL).title("Description"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            area,
        );
    }