use crate::rules::target::Target;
use crate::rules::rule::{Rule, Action};
use std::io::Error;
use std::fs;

pub struct Engine{
    rules: Vec<Rule>,
    fileinfo: String,
}

impl Engine {
    pub fn new(rules: Vec<Rule>) -> Result<Self, Error>{
        let fileinfo = fs::read_to_string("/etc/hosts")?;
        Ok(Self{
            rules,
            fileinfo,
        })
    }

    // pub fn check(&self, target: &Target) -> Action{
    //     for rule in &self.rules{
    //         if Engine::matches(&rule.target, target){
    //             return rule.action.clone();
    //         }
    //     }
    //     Action::Allow
    // }

    fn matches(rule_target: &Target, input: &Target) -> bool{
        match(rule_target, input){
            (Target::Website(a), Target::Website(b)) => a == b,
            (Target::Application(a), Target::Application(b)) => a == b,
            _ => false,
        }
    }
}