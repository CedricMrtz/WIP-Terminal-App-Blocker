use crate::rules::target::Target;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Action{
    Block,
    Allow,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub target: Target,
    pub action: Action,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target = match &self.target {
            Target::Website(url) => format!("Website: {}", url),
            Target::Application(app) => format!("App: {}", app),
        };

        write!(f, "{} → {}", self.name, target)
    }
}