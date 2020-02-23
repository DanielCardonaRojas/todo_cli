use clap::Clap;

#[derive(Clap)]
pub enum Command {
    #[clap(about = "Lists all pending tasks")]
    Get,
    #[clap(about = "Add a new task to todo list")]
    Add(Add),
    #[clap(about = "Toggle complete on task")]
    Done(Done),
    Remove(Remove),
}

#[derive(Clap)]
pub struct Add {
    pub description: String,
}

#[derive(Clap, Debug)]
pub struct Done {
    pub index: usize,
}

#[derive(Clap)]
pub struct Remove {
    pub index: usize,
}
