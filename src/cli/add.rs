use clap::{arg, ArgAction, ArgMatches, Command};

use crate::tasks::{path::resolve_path, Rukefile};
use colorized::{Color, Colors};

pub fn add_command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(arg!(-n --name <NAME> "Set the task name"))
        .arg(arg!(-c --command <COMMAND>"Set the task command").action(ArgAction::Append))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("a")
}

pub fn add_handler(matches: &ArgMatches) {
    let filepath = matches.get_one::<String>("file");

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!("{}", "Rukefile not found".color(Colors::RedFg));
            return;
        }
    };

    let rukefile = Rukefile::new(filepath.clone());

    if let Err(e) = rukefile {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    let task_name = matches.get_one::<String>("name");
    let task_command = matches.get_one::<String>("command");

    let mut rukefile = rukefile.unwrap();

    if task_name.is_none() {
        eprintln!("{}", "The task must have a name.".color(Colors::RedFg));
        return;
    }

    if task_command.is_none() {
        eprintln!("{}", "The task must have a command.".color(Colors::RedFg));
        return;
    }

    let task_name = task_name.unwrap();
    let task_command = task_command.unwrap();

    if let Err(e) = rukefile.add_task(task_name.to_string(), task_command.to_string()) {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    if let Err(e) = rukefile.update_rukefile(filepath) {
        eprintln!("{:?}", e);
        return;
    }

    println!("{}", "Task added successfully!".color(Colors::GreenFg));
}
