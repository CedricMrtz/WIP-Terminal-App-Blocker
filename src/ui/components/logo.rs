use ratatui::{
    Frame, layout::{Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, Paragraph}
};

pub fn render_logo(frame: &mut Frame, area: Rect){
        
        let logo = Paragraph::new(
            Line::from(vec![
                Span::styled("Logo", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ])
        );

        frame.render_widget(
            logo
                .block(Block::new().borders(Borders::ALL))
                .style(Style::default().fg(Color::Red)),
            area,
        );
    }