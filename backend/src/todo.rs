use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl ToString for RepeatRule {
    fn to_string(&self) -> String {
        match self {
            RepeatRule::Never => String::from("Never"),
            RepeatRule::Daily => String::from("Daily"),
            RepeatRule::Weekly => String::from("Weekly"),
            RepeatRule::Monthly => String::from("Monthly"),
            RepeatRule::Yearly => String::from("Yearly"),
        }
    }
}

impl From<String> for RepeatRule {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Daily" => RepeatRule::Daily,
            "Weekly" => RepeatRule::Weekly,
            "Monthly" => RepeatRule::Monthly,
            "Yearly" => RepeatRule::Yearly,
            _ => RepeatRule::Never,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Utc>>,
    pub reminder: Vec<DateTime<Utc>>,
    pub repeat: RepeatRule,
    pub completed: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTodo {
    pub title: String,
    pub description: String,
    pub due: Option<DateTime<Utc>>,
    pub reminder: Vec<DateTime<Utc>>,
    pub repeat: RepeatRule,
}
