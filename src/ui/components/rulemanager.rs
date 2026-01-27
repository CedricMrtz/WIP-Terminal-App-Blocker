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

        let container = Block::new()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(Color::Red));

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
            frame.render_widget(container.clone(), area);
            let inner = container.inner(area);

            let editmenu = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(48),
                    Constraint::Length(1),
                    Constraint::Percentage(52),
                ])
                .split(inner);

            frame.render_widget(Paragraph::new("Edit")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Green)),
            editmenu[0]);

            frame.render_widget(Block::new().
                borders(Borders::TOP), 
                editmenu[1]
            );

            frame.render_widget(Paragraph::new("Delete")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Green)),
            editmenu[2]);
        }

    }