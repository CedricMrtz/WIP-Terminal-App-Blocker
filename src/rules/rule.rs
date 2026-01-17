use crate::rules::target::Target;

#[derive(Debug, Clone)]
pub enum Action{
    Block,
    Allow,
}

pub struct Rule {
    pub name: String,
    pub target: Target,
    pub action: Action,
}