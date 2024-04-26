use crate::tasks::{path::resolve_path, Rukefile};
use clap::{arg, ArgMatches, Command};

use colorized::{Color, Colors};
pub fn remove_command() -> Command {
    Command::new("remove")
        .about("Remove a existing task in recipe")
        .arg(arg!(-n --name <NAME> "Task name to delete"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .alias("rm")
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

    let mut rukefile = rukefile.unwrap();

    if let Some(name) = task_name {
        if let Err(e) = rukefile.remove_task(name.to_string()) {
            eprintln!("{}", e.color(Colors::RedFg));
            return;
        }
        if let Err(e) = rukefile.update_rukefile(filepath) {
            eprintln!("{:?}", e);
            return;
        }
    }

    println!("{}", "Task removed successfully!".color(Colors::GreenFg));
}
