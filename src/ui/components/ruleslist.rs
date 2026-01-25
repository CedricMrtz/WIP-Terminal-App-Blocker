use ratatui::{
    Frame, layout::{Alignment, Rect},
    style::{Color, Style, Modifier},
    text::{Line},
    widgets::{Block, Borders, List, ListItem}
};
use crate::app::App;

pub fn render_ruleslist(frame: &mut Frame, area: Rect, app: &App){
    
        let items: Vec<Line> = app.rules
        .iter() //Iter by reference
        .map(|r| Line::from(r.to_string()))
        .collect();

        let list = List::new(items)
                .block(Block::new().borders(Borders::ALL).title("Rules"))
                .style(Style::default().fg(Color::Yellow))
                .highlight_style(
                    Style::default()
                    .bg(Color::Black)
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                )
                .highlight_symbol(">>> ");

        frame.render_stateful_widget(list, area, &mut app.ruleslist_state.clone());
    }