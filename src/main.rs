mod engine;
mod rule;
mod target;
mod ui;

use engine::Engine;
use rule::{Rule, Action};
use target::Target;
use ui::App;

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

    color_eyre::install()?;
    let terminal = ratatui::init();
    App::new().run(terminal, text)?;
    ratatui::restore();
    Ok(())
}