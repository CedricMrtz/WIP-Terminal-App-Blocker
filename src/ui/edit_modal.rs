use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Alignment, Rect},
    widgets::{Block, Borders, Paragraph},
};
use crate::{app::App};
use crate::ui::{main_screen};


pub fn render_edit_modal(frame: &mut Frame, app: &mut App){
    let area = centered_rect(40, 30, frame.size());

    main_screen::render_main(frame, app);

    frame.render_widget(Paragraph::new("Edit or delete the selected rule")
        .block(Block::new()
        .borders(Borders::ALL)
        .title("Edit Rule Modal")
        .title_alignment(Alignment::Center))
    ,area);

    }

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}