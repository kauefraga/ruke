use clap::{arg, ArgMatches, Command};
use colorized::{Color, Colors};

use crate::tasks::{path::resolve_path, Rukefile};

pub fn list_command() -> Command {
    Command::new("list")
        .about("List existing tasks")
        .arg(arg!(-m --minimal "Print the tasks name only"))
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

    let rukefile = Rukefile::new(filepath.clone());

    if let Err(e) = rukefile {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    let rukefile = rukefile.unwrap();

    let tasks = rukefile.tasks.iter();
    let minimal = matches.get_one("minimal").unwrap_or(&false);

    if *minimal {
        let mut names = Vec::<String>::new();
        tasks.for_each(|(name, _)| names.push(name.color(Colors::BlueFg)));

        println!("{}", names.join(", "));
        return;
    }

    println!("Ruke file tasks ({} found):", rukefile.tasks.len());

    tasks.for_each(|(name, task)| println!("{}", task.display(name)));
}
