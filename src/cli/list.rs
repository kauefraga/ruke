use clap::{arg, ArgMatches, Command};
use colorized::{Color, Colors};

use crate::tasks::{path::resolve_path, Rukefile};

pub fn list_command() -> Command {
    Command::new("list")
        .about("List the name of existing tasks")
        .arg(arg!(-a --all "Show all details of each task"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("ls")
}

pub fn list_handler(matches: &ArgMatches) {
    let filepath = matches.get_one::<String>("file");

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!("{}", "Ruke file not found.".color(Colors::RedFg));
            return;
        }
    };

    let rukefile = Rukefile::new(filepath);

    if let Err(e) = rukefile {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    let rukefile = rukefile.unwrap();

    let tasks = rukefile.tasks.iter();

    if *matches.get_one::<bool>("all").unwrap_or(&false) {
        tasks
            .into_iter()
            .for_each(|(name, task)| println!("{} ", task.display(name)));
        return;
    }

    tasks
        .into_iter()
        .for_each(|(name, _)| println!("{}", name.color(Colors::GreenFg)))
}
