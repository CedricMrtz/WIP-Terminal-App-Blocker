mod app;
mod ui;
mod rules;

// use rules::engine::Engine;
use rules::rule::{Rule, Action};
use rules::target::Target;
use app::App;

fn main() -> color_eyre::Result<()> {
    let rules = vec![
        Rule {
            name: "Work focus".into(),
            target: vec![
                Target::Website("youtube.com".into()),
                Target::Application("Steam".into())
            ],
            action: Action::Block,
            description: Some("Focus for working".into()),
            time_permitted: 100,
            time_remaining: 100,
        },
        Rule{
            name: "Group 2".into(),
            target: vec![
                Target::Application("firefox".into()),
            ],
            action: Action::Allow,
            description: None,
            time_permitted: 0,
            time_remaining: 0,
        }
    ];
    
    // let engine = Engine::new(rules.clone());

    // let input = Target::Website("youtube.com".into());

    // let result = engine.check(&input);

    // let text = match result {
    //     Action::Allow => "Access Allowed",
    //     Action::Block => "Access Blocked",
    // };
    let text: &str = "Feature yet to make";

    let mut app = App::new();
    app.status_message = text.into(); 
    app.rules = rules.into();

    color_eyre::install()?;
    let terminal = ratatui::init();
    app.run(terminal)?;
    ratatui::restore();
    Ok(())
}