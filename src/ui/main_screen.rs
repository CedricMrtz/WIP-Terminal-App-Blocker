use ratatui::{
    Frame,
    layout::{Layout, Direction, Constraint, Alignment},
    widgets::{Block, Borders, Paragraph},
    style::{Style, Color},
};
use crate::{app::App, rules::rule};
use crate::ui::components::{logo, rulesdescription, ruleslist, rulemanager};


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
                Constraint::Percentage(40),
                Constraint::Percentage(60),
            ])
            .split(inner);

        logo::render_logo(frame, outer[0]);

        let info = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(outer[1]);

        ruleslist::render_ruleslist(frame, info[0], app);

        rulesdescription::render_rulesdescription(frame, info[1], app);

        rulemanager::render_rulemanager(frame, info[2], app);

    }