pub enum Command {
    Get,
    Add(String),
    Done(usize),
    Remove(usize),
}

impl Command {}

impl From<Vec<String>> for Command {
    fn from(args: Vec<String>) -> Self {
        match args[1].clone().as_str() {
            "get" => Command::Get,
            "add" => Command::Add(args[2].clone()),
            "done" => Command::Done(args[2].parse().unwrap()),
            "remove" => Command::Remove(args[2].parse().unwrap()),
            _ => panic!("Must provide a proper command"),
        }
    }
}
