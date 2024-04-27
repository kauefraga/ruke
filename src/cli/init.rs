use std::fs;

use clap::{ArgMatches, Command};
use colorized::{Color, Colors};
use serde::Serialize;

use crate::tasks::{path::resolve_path, Rukefile, Task};

pub fn init_command() -> Command {
    Command::new("init")
        .about("Create a Ruke.toml file with a task within")
        .alias("i")
}

pub fn init_handler(_matches: &ArgMatches) {
    if resolve_path(None).is_some() {
        println!("{}", "A ruke file already exists".color(Colors::YellowFg));
        return;
    }

    let example_task = Task {
        name: "main".to_string(),
        command: "echo Hello, Ruke!".to_string(),
        arguments: None,
    };

    let rukefile = Rukefile {
        tasks: vec![example_task],
    };

    let mut rukefile_toml = String::new();

    rukefile
        .serialize(toml::Serializer::new(&mut rukefile_toml))
        .unwrap();

    fs::write("Ruke.toml", &rukefile_toml).unwrap();

    println!(
        "Ruke.toml file {}!\n",
        "created successfully".color(Colors::GreenFg)
    );
    println!("{}", &rukefile_toml);
    println!(
        "Try `{}` and `{}`.\nHappy hacking!",
        "ruke list".color(Colors::BlueFg),
        "ruke add".color(Colors::BlueFg)
    );
}
