use super::todo_item::*;
use reqwest::blocking::*;

pub trait DataManager {
    fn list(&self) -> Vec<TodoItem>;
    fn create(&self, todo: String) -> TodoItem;
    fn remove(&mut self, todo: TodoItem);
    fn mark(&self, todo: &mut TodoItem, complete: bool);
}

pub struct RemoteDataManager {
    pub base_url: String,
    pub client: Client,
}

impl RemoteDataManager {
    pub fn new(base_url: String) -> RemoteDataManager {
        return RemoteDataManager {
            base_url: base_url,
            client: Client::new(),
        };
    }
}

impl DataManager for RemoteDataManager {
    fn list(&self) -> Vec<TodoItem> {
        let task_data = self
            .client
            .get(&self.base_url)
            .send()
            .expect("Something wrong fetching data from jsongplaceholder.typicode.com");

        let tasks: Vec<TodoItem> = task_data.json().expect("Couldn't parse todo");
        return tasks;
    }

    fn create(&self, new_item: String) -> TodoItem {
        let todo = TodoItem::new(new_item);
        let json_string = serde_json::to_string(&todo).expect("Could not serialize");
        let client = &self.client;
        let response = client
            .post(&self.base_url)
            .header("Content-Type", "application/json")
            .body(json_string)
            .send()
            .expect("Couldn't create new todo");

        let created_todo: TodoItem = response
            .json()
            .expect("Couldn't parse response to TodoItem");

        return created_todo;
    }

    fn remove(&mut self, todo: TodoItem) {
        let delete_url = &todo.id;
        let response = self.client.delete(delete_url).send().unwrap();

        if response.status().is_success() {
            println!("Removed task: {}", todo);
        } else {
            println!("Failed to remove task: {}", todo);
        }
    }

    fn mark(&self, task: &mut TodoItem, complete: bool) {
        task.completed = complete;
        let json_string = serde_json::to_string(&task).expect("Could not serialize");
        let patch_url = &task.id;
        let response = self
            .client
            .patch(patch_url)
            .header("Content-Type", "application/json")
            .body(json_string)
            .send()
            .expect("Couldn't patch todo");
        let patched_todo: Result<TodoItem, reqwest::Error> = response.json();

        if let Some(parsed) = patched_todo.ok() {
            println!("Updated {}", parsed);
        }
    }
}
