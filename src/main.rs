mod command;
mod data_manager;
mod todo_item;
mod todo_list;

use clap;
use clap::Clap;
use command::*;
use data_manager::*;
use std::env;
use std::fmt::Debug;
use todo_item::*;
use todo_list::*;

#[derive(Clap)]
#[clap(version = "1.0", author = "Daniel Cardona R.")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Command,
}

fn main() {
    let opts: Opts = Opts::parse();

    let url = "https://todo-backend-rocket-rust.herokuapp.com";
    let mut remote_data_manager = RemoteDataManager::new(url.to_string());

    let list = remote_data_manager.list();
    let mut todo_list = TodoList { tasks: list };

    return match opts.subcmd {
        Command::Get => {
            let list = remote_data_manager.list();
            let list = TodoList { tasks: list };
            todo_list = list;
            todo_list.print();
        }
        Command::Add(cmd) => {
            let created_todo = remote_data_manager.create(cmd.description);
            todo_list.add(created_todo);
            todo_list.print();
        }
        Command::Done(cmd) => {
            let mut task = todo_list.tasks[cmd.index].clone();
            let done = task.completed;
            remote_data_manager.mark(&mut task, !done);
        }
        Command::Remove(cmd) => {
            let task = todo_list.tasks[cmd.index].clone();
            remote_data_manager.remove(task);
        }
    };
}
