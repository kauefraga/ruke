use crate::tasks::{path::resolve_path, Rukefile};
use clap::{arg, ArgMatches, Command};

use colorized::{Color, Colors};
pub fn remove_command() -> Command {
    Command::new("remove")
        .about("Remove an existing task")
        .arg(arg!(-n --name <NAME> "Set the task name"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("rm")
}

pub fn remove_handler(matches: &ArgMatches) {
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

    let mut rukefile = rukefile.unwrap();

    let task_name = matches.get_one::<String>("name");

    if task_name.is_none() {
        eprintln!("{}", "The task must have a name.".color(Colors::RedFg));
        println!("Try adding `{}`.", "--name task-name".color(Colors::BlueFg));
        return;
    }

    let task_name = task_name.unwrap();

    if let Err(e) = rukefile.remove_task(task_name.to_string()) {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    if let Err(e) = rukefile.update_rukefile(filepath) {
        eprintln!("{:?}", e);
        return;
    }

    println!("{}", "Task removed successfully!".color(Colors::GreenFg));
}
