mod engine;
mod rule;
mod target;

use engine::Engine;
use rule::{Rule, Action};
use target::Target;

fn main(){
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

    let input = Target::App("firefox".into());

    let result = engine.check(&input);

    match result {
        Action::Allow => println!("Access Allowed"),
        Action::Block => println!("Access Blocked"),
    }
}