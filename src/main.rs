extern crate reqwest;
extern crate serde;

mod command;
mod todo_item;
mod todo_list;
use command::*;
use reqwest::header::HeaderName;
use serde_json;
use std::env;
use std::result::*;
use todo_item::*;
use todo_list::*;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = Command::from(arguments);

    // let url = "https://jsonplaceholder.typicode.com/todos";
    let url = "https://todo-backend-rocket-rust.herokuapp.com";

    let client = reqwest::blocking::Client::new();
    let task_data = client
        .get(url)
        .send()
        .expect("Something wrong fetching data from jsongplaceholder.typicode.com");

    let tasks: Vec<TodoItem> = task_data.json().expect("Couldn't parse todo");

    let mut todo_list = TodoList { tasks: tasks };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(new_item) => {
            let new_todo = TodoItem::new(new_item);
            let json_string = serde_json::to_string(&new_todo).expect("Could not serialize");
            let response = client
                .post("https://todo-backend-rocket-rust.herokuapp.com/")
                .header("Content-Type", "application/json")
                .body(json_string)
                .send()
                .expect("Couldn't create new todo");

            let created_todo: TodoItem = response
                .json()
                .expect("Couldn't parse response to TodoItem");
            todo_list.add(created_todo);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_item(index, true);
            todo_list.print();
        }
        Command::Remove(index) => {
            let task = todo_list.tasks[index].clone();
            let delete_url = &task.id;
            let response = client.delete(delete_url).send().unwrap();
            if response.status().is_success() {
                todo_list.remove(index);
                println!("Removed task: {}", task);
            } else {
                println!("Failed to remove task: {}", task);
            }
        }
    }
}
