use std::path::PathBuf;

use clap::{arg, ArgMatches, Command};
use inquire::{required, Text};

use crate::tasks::{path::resolve_path, Rukefile};
use colorized::{Color, Colors};

pub fn new_command() -> Command {
    Command::new("new")
        .about("Create a new task")
        .arg(arg!(-n --name <NAME> "Set the task name"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("n")
}

pub fn new_handler(matches: &ArgMatches) {
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
        let task_name = Text::new("Task name:")
            .with_validator(required!("The task's name is required"))
            .prompt();
        if let Err(e) = task_name {
            println!("{}", format!("{}.", e).color(Colors::RedFg));
            return;
        }

        let task_name = task_name.unwrap();

        if let Err(e) = create_task_and_update_rukefile(&mut rukefile, filepath, task_name) {
            return eprintln!("{}", e.color(Colors::RedFg));
        }

        println!("{}", "Task added successfully!".color(Colors::GreenFg));

        return;
    }

    let task_name = task_name.unwrap();

    if let Err(e) = create_task_and_update_rukefile(&mut rukefile, filepath, task_name.to_string())
    {
        return eprintln!("{}", e.color(Colors::RedFg));
    }

    println!("{}", "Task added successfully!".color(Colors::GreenFg));
}

fn create_task_and_update_rukefile(
    rukefile: &mut Rukefile,
    filepath: PathBuf,
    task_name: String,
) -> Result<(), String> {
    rukefile.create_task(task_name)?;

    if let Err(e) = rukefile.update_rukefile(filepath) {
        return Err(e.to_string());
    }

    Ok(())
}
