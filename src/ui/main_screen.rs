use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Alignment},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color, Modifier},
    text::{Line, Span},
};
use crate::app::App;
use crate::ui::components::ascii;


pub fn render_main(frame: &mut Frame, app: &App){
        let area = frame.area();

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Terminal App Blocker")
            .title_alignment(Alignment::Center);

        frame.render_widget(block.clone(), area);
        let inner = block.inner(area);

        //  ascii::render_logo(frame, inner);

        let outer = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(8),
                Constraint::Min(0),
            ])
            .split(inner);

        ascii::render_logo(frame, outer[0]);

        frame.render_widget(
            Paragraph::new(app.status_message.as_str())
                .block(Block::new().borders(Borders::ALL).title("Status"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Yellow)),
            outer[1],
        );

    }