use serde::{Deserialize, Serialize};
use std::fmt;
use std::result::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    #[serde(rename = "title")]
    name: String,
    completed: bool,
}

impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: false,
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

pub struct TodoList {
    pub tasks: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        return TodoList { tasks: vec![] };
    }

    pub fn add(&mut self, todo: TodoItem) {
        self.tasks.push(todo);
    }

    pub fn remove(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn print(&self) {
        for item in &self.tasks {
            println!("{}", item);
        }
    }

    pub fn mark_item(&mut self, index: usize, complete: bool) {
        self.tasks[index].completed = complete;
    }
}
