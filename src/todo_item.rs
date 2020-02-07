use serde::{Deserialize, Serialize};
use std::fmt;
use std::result::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    #[serde(rename = "title")]
    pub name: String,

    pub completed: bool,

    #[serde(rename = "url")]
    pub id: String,
}

impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: false,
            id: "".to_string(),
        };
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let complete_sign = match self.completed {
            true => 'x',
            false => ' ',
        };
        return write!(f, "[{}] - {}", complete_sign, self.name);
    }
}
