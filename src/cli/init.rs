use std::{collections::HashMap, fs};

use clap::{ArgMatches, Command};
use colorized::{Color, Colors};

use crate::tasks::{path::resolve_path, Rukefile, Task};

pub fn init_command() -> Command {
    Command::new("init")
        .about("Create a Ruke.toml file with a task within")
        .alias("i")
}

pub fn init_handler(_matches: &ArgMatches) {
    if resolve_path(None).is_some() {
        eprintln!("{}", "A ruke file already exists.".color(Colors::RedFg));
        return;
    }

    let example_task = Task {
        commands: Some(vec![
            "echo Hello, Ruke!".to_string(),
            "echo !ekuR ,ollhH".to_string(),
        ]),
    };

    let mut tasks = HashMap::new();
    tasks.insert("main".to_string(), example_task);

    let rukefile = Rukefile { tasks };
    let rukefile = toml::to_string(&rukefile).unwrap();

    fs::write("Ruke.toml", &rukefile).unwrap();

    println!(
        "Ruke.toml file {}!\n",
        "created successfully".color(Colors::GreenFg)
    );
    println!("{}", &rukefile);
    println!(
        "Try `{}` and `{}`.\nHappy hacking!",
        "ruke list".color(Colors::BlueFg),
        "ruke add".color(Colors::BlueFg)
    );
}
