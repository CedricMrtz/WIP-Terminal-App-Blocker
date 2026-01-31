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
    pub target: Vec<Target>,
    pub action: Action,
    pub description: Option<String>,
    pub time_permitted: u32,
    pub time_remaining: u32,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let targets: Vec<String> = self.target.iter().map(|t| match t {
        //     Target::Website(url) => format!("Website: {}", url),
        //     Target::Application(app) => format!("App: {}", app),
        // }).collect();
        write!(f, "{}", self.name)
    }
}