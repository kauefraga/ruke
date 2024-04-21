use clap::{arg, Command};

pub fn list() -> Command {
    Command::new("list")
        .about("List the name of existing tasks")
        .arg(arg!(-a --all "Show all details of each task"))
        .alias("ls")
}
