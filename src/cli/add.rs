use std::path::PathBuf;

use clap::{arg, ArgAction, ArgMatches, Command};
use inquire::{required, Text};

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
            add_task_and_update_rukefile(&mut rukefile, filepath, task_name, task_command)
        {
            return eprintln!("{}", e.color(Colors::RedFg));
        };

        println!("{}", "Task added successfully!".color(Colors::GreenFg));

        return;
    }
    if task_name.is_none() {
        eprintln!("{}", "The task must have a name.".color(Colors::RedFg));
        println!("Try adding `{}`.", "--name task-name".color(Colors::BlueFg));
        return;
    }

    if task_command.is_none() {
        eprintln!("{}", "The task must have a command.".color(Colors::RedFg));
        println!(
            "Try adding `{}`.",
            "--command 'task command and its arguments'".color(Colors::BlueFg)
        );
        return;
    }

    let task_name = task_name.unwrap();
    let task_command = task_command.unwrap();

    if let Err(e) = add_task_and_update_rukefile(
        &mut rukefile,
        filepath,
        task_name.to_string(),
        task_command.to_string(),
    ) {
        return eprintln!("{}", e.color(Colors::RedFg));
    }

    println!("{}", "Task added successfully!".color(Colors::GreenFg));
}

fn add_task_and_update_rukefile(
    rukefile: &mut Rukefile,
    filepath: PathBuf,
    task_name: String,
    task_command: String,
) -> Result<(), String> {
    rukefile.add_task(task_name, task_command)?;

    if let Err(e) = rukefile.update_rukefile(filepath) {
        return Err(e.to_string());
    }
    Ok(())
}
