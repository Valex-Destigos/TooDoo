use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RepeatRule {
    Never,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Default for RepeatRule {
    fn default() -> Self {
        RepeatRule::Never
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Local>>,
    pub reminder: Vec<DateTime<Local>>,
    pub repeat: RepeatRule,
    pub completed: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Local>>,
    pub reminder: Vec<DateTime<Local>>,
    pub repeat: RepeatRule,
}
