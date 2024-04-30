use std::path::PathBuf;

use clap::{arg, ArgMatches, Command};
use colorized::{Color, Colors};
use inquire::{required, Text};

use crate::tasks::{path::resolve_path, Rukefile};

pub fn add_command() -> Command {
    Command::new("add")
        .about("Add a command to an existing task")
        .arg(arg!(-n --name <NAME> "Set the task name"))
        .arg(arg!(-c --command <COMMAND> "Set the task command"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("a")
}

pub fn add_handler(matches: &ArgMatches) {
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
    let task_command = matches.get_one::<String>("command");

    if task_name.is_none() && task_command.is_none() {
        let task_name = Text::new("Task name:")
            .with_validator(required!("The task's name is required"))
            .prompt();
        let task_command = Text::new("Task command:")
            .with_validator(required!("The task's command is required"))
            .prompt();
        let task_name = task_name.unwrap();
        let task_command = task_command.unwrap();

        if let Err(e) =
            add_command_and_update_tasks(&mut rukefile, filepath, task_name, task_command)
        {
            eprintln!("{}", e.color(Colors::RedFg));
            return;
        }

        println!("{}", "Command added successfully!".color(Colors::GreenFg));
        return;
    }

    let task_name = task_name.unwrap();
    let task_command = task_command.unwrap();

    if let Err(e) = add_command_and_update_tasks(
        &mut rukefile,
        filepath,
        task_name.to_string(),
        task_command.to_string(),
    ) {
        eprintln!("{}", e.color(Colors::RedFg));
    }

    println!("{}", "Command added successfully!".color(Colors::GreenFg));
}

fn add_command_and_update_tasks(
    rukefile: &mut Rukefile,
    filepath: PathBuf,
    task_name: String,
    task_command: String,
) -> Result<(), String> {
    rukefile.add_command(task_name, task_command)?;

    if let Err(e) = rukefile.update_rukefile(filepath) {
        return Err(e.to_string());
    }
    Ok(())
}
