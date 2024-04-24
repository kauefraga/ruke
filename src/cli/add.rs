use clap::{arg, ArgAction, ArgMatches, Command};

use crate::tasks::{path::resolve_path, Rukefile};
use colorized::{Color, Colors};

pub fn add_command() -> Command {
    Command::new("add")
        .about("Add a new task")
        .arg(arg!(-n --name <NAME> "Sets task name"))
        .arg(arg!(-c --command <COMMAND>"Set a command in a task").action(ArgAction::Append))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("a")
}

pub fn add_handler(matches: &ArgMatches) {
    let filepath = matches.get_one::<String>("file");

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!("{}", "rukefile not found".color(Colors::RedFg));
            return;
        }
    };

    let rukefile = Rukefile::new(filepath.clone());

    if let Err(e) = rukefile {
        eprintln!("{:?}", e);
        return;
    }

    let task_name = matches.get_one::<String>("name");
    let command = matches.get_one::<String>("command");

    let mut rukefile = rukefile.unwrap();

    if let Some(name) = task_name {
        if let Some(cmd) = command {
            {
                if let Err(e) = rukefile.add_task(name.to_string(), cmd.to_string()) {
                    eprintln!("{} {}", name.color(Colors::RedFg), e.color(Colors::RedFg));
                    return;
                };
                if let Err(e) = rukefile.update_rukefile(filepath) {
                    eprintln!("{:?}", e);
                    return;
                }
            };
        }
    }
    println!("{}", "Task added successfully!".color(Colors::GreenFg));
}
