use clap::{arg, Command};

pub fn list() -> Command {
    Command::new("list")
        .about("List tasks in recipe")
        .arg(arg!(-a --all "List all tasks"))
        .alias("ls")
}
