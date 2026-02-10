use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Rect},
    buffer::Buffer,
    widgets::{Block, Borders, Paragraph, Widget},
    style::{Style, Color},
};
use crate::{app::App};

pub fn render_delete_modal(frame: &mut Frame, app: &mut App){
    let area = centered_rect(40, 30, frame.size());
    
    let mut overlay = Buffer::empty(area);
    
    let block = Block::new()
        .title("Delete Rule")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    block.render(area, &mut overlay);

    frame.buffer_mut().merge(&overlay);
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