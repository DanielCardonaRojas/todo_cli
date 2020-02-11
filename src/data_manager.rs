extern crate reqwest;
extern crate serde;

use super::command::*;
use super::todo_item::*;
use super::todo_list::*;

trait DataManager {
    fn list(&self) -> Vec<TodoItem>;
    fn create(&self, todo: String) -> TodoItem;
    fn remove(&self todo: TodoItem);
    fn mark(&self todo: TodoItem, complete: bool); 
}

pub struct RemoteDateManager {
    pub base_url: String,
    pub client: Client = reqwest::blocking::Client::new(),

}

impl DataManager for RemoteDateManager {
    fn list(&self) -> Vec<TodoItem> {
        return vec![];

    }

    fn create(&self, new_item: String) -> TodoItem {
        let todo = TodoItem::new(new_item);
        let json_string = serde_json::to_string(&todo).expect("Could not serialize");
        let response = &self.client
            .post(self.base_url)
            .header("Content-Type", "application/json")
            .body(json_string)
            .send()
            .expect("Couldn't create new todo");

        let created_todo: TodoItem = response
        .json()
        .expect("Couldn't parse response to TodoItem")

        return created_todo;

    }

    fn remove(&self, todo: TodoItem) {

    }

    fn mark(&self, task: &self TodoItem, complete: bool) {
        let done = task.completed;
        task.completed = !done;
        let json_string = serde_json::to_string(&task).expect("Could not serialize");
        let patch_url = &task.id;
        let response = self.client
            .patch(patch_url)
            .header("Content-Type", "application/json")
            .body(json_string)
            .send()
            .expect("Couldn't patch todo");

    } 

}
