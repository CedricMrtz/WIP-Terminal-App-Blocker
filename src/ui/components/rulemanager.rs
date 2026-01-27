use ratatui::{
    Frame,
    layout::{Alignment, Rect, Layout, Direction, Constraint},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph}
};
use crate::app::App;

pub fn render_rulemanager(frame: &mut Frame, area: Rect, app: &App){
        
        let title = app.selected_rule
            .as_ref()
            .map(|r| r.name.as_str())
            .unwrap_or("No rule selected");

        let editmenu = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        if title == "No rule selected"{
            frame.render_widget(
                Paragraph::new("Info")
                    .block(Block::new().borders(Borders::ALL).title(title)
                        .title_alignment(Alignment::Left))
                    .alignment(Alignment::Left)
                    .style(Style::default().fg(Color::Red)),
                area,
            );
        }else{
            frame.render_widget(Paragraph::new("Edit rule")
                .block(Block::new().borders(Borders::ALL).title(title)
                    .title_alignment(Alignment::Left))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            editmenu[0]);

            frame.render_widget(Paragraph::new("Delete rule")
                .block(Block::new().borders(Borders::ALL).title(title)
                    .title_alignment(Alignment::Left))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            editmenu[1]);
        }

    }