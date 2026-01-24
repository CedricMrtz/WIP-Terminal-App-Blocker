#[derive(Debug, Clone)]
pub enum Target {
    Website(String),
    Application(String),
}