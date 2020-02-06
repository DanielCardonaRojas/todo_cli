extern crate reqwest;
extern crate serde;

mod command;
mod todo_item;
mod todo_list;
use command::*;
use std::env;
use std::result::*;
use todo_item::*;
use todo_list::*;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = Command::from(arguments);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://jsonplaceholder.typicode.com/todos")
        .send()
        .expect("Something wrong fetching data from jsongplaceholder.typicode.com");

    let tasks: Vec<TodoItem> = response.json().expect("Couldn't parse todo");

    let mut todo_list = TodoList { tasks: tasks };

    match command {
        Command::Get => todo_list.print(),
        Command::Add(new_todo) => {
            todo_list.add(TodoItem::new(new_todo));
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_item(index, true);
            todo_list.print();
        }
        Command::Remove(index) => {
            todo_list.remove(index);
            todo_list.print();
        }
    }
}
