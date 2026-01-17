mod app;
mod ui;
mod rules;

use rules::engine::Engine;
use rules::rule::{Rule, Action};
use rules::target::Target;
use app::App;

fn main() -> color_eyre::Result<()> {
    let rules = vec![
        Rule {
            name: "Block Youtube".into(),
            target: Target::Website("youtube.com".into()),
            action: Action::Block,
        },
        Rule{
            name: "Allow firefox".into(),
            target: Target::App("firefox".into()),
            action: Action::Allow,
        }
    ];
    
    let engine = Engine::new(rules);

    let input = Target::Website("youtube.com".into());

    let result = engine.check(&input);

    let text = match result {
        Action::Allow => "Access Allowed",
        Action::Block => "Access Blocked",
    };

    let mut app = App::new();
    app.status_message = text.into(); 

    color_eyre::install()?;
    let terminal = ratatui::init();
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}