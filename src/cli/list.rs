use clap::{arg, ArgMatches, Command};

use crate::tasks::Rukefile;

pub fn list_command() -> Command {
    Command::new("list")
        .about("List the name of existing tasks")
        .arg(arg!(-a --all "Show all details of each task"))
        .alias("ls")
}

pub fn list_handler(matches: &ArgMatches, rukefile: Rukefile) {
    if *matches.get_one::<bool>("all").unwrap_or(&false) {
        rukefile.all_tasks();
        return;
    }

    rukefile.list_tasks();
}
