use super::todo_item::*;

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
        for (i, item) in self.tasks.iter().enumerate() {
            println!("{} - {}", i, item);
        }
    }

    pub fn mark_item(&mut self, index: usize, complete: bool) {
        self.tasks[index].completed = complete;
    }
}
