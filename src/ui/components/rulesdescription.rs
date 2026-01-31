use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph}
};
use crate::app::App;
use crate::rules::target::Target;

pub fn render_rulesdescription(frame: &mut Frame, area: Rect, app: &App){
        
        let selected_rule = app
            .rules_selection
            .selected()
            .and_then(|i| app.rules.get(i));

        let descriptions: Vec<String> = vec![
            format!(
                "Apps: {}",
                selected_rule
                    .map(|r| r.target.as_ref())
                    .map(|targets: &Vec<Target>| {
                        targets
                            .iter()
                            .map(|t| match t{
                                Target::Website(url) => format!("{}", url),
                                Target::Application(app) => format!("{}", app),
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "None".into())
            ),
            format!(
                "Time permitted: {} minutes",
                selected_rule
                    .map(|r| r.time_permitted)
                    .unwrap_or(0)
            ),
            format!(
                "Description: {}",
                selected_rule
                    .and_then(|r| r.description.as_ref())
                    .map(|d| d.as_str())
                    .unwrap_or("No description provided")
            ),
            format!(
                "\nTime remaining: {} minutes",
                selected_rule
                    .map(|r| r.time_remaining)
                    .unwrap_or(0)
            ),
        ];
        
        frame.render_widget(
            Paragraph::new(descriptions.join("\n"))
                .block(Block::new().borders(Borders::ALL).title("Description"))
                .alignment(Alignment::Left)
                .style(Style::default().fg(Color::Green)),
            area,
        );
    }