mod command;
mod data_manager;
mod todo_item;
mod todo_list;

use command::*;
use data_manager::*;
use std::env;
use todo_item::*;
use todo_list::*;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = Command::from(arguments);

    let url = "https://todo-backend-rocket-rust.herokuapp.com";
    let mut remote_data_manager = RemoteDataManager::new(url.to_string());

    let mut todo_list = TodoList { tasks: vec![] };

    match command {
        Command::Get => {
            let list = remote_data_manager.list();
            let list = TodoList { tasks: list };
            todo_list = list;
            todo_list.print();
        }
        Command::Add(new_item) => {
            let created_todo = remote_data_manager.create(new_item);
            todo_list.add(created_todo);
            todo_list.print();
        }
        Command::Done(index) => {
            let mut task = todo_list.tasks[index].clone();
            let done = task.completed;
            remote_data_manager.mark(&mut task, !done);
        }
        Command::Remove(index) => {
            let task = todo_list.tasks[index].clone();
            remote_data_manager.remove(task);
        }
    }
}
