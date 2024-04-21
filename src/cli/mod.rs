pub mod list;

use colorized::{Color, Colors};
use clap::{arg, ArgMatches, Command};

use crate::tasks::{resolve_path, Rukefile};

pub fn root_command() -> Command {
    Command::new("ruke")
        .author("Kauê Fraga Rodrigues")
        .version("0.1.0")
        .about("A dead-simple automation tool. Inspired by Makefile and Justfile.")
        .arg(arg!([target] "Set the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Set run to be silent"))
        .subcommand(list::list_command())
}

pub fn root_handler(matches: ArgMatches) {
    let target = matches.get_one::<String>("target").unwrap();
    let filepath = matches.get_one::<String>("file");
    let quiet = matches.get_one::<bool>("quiet").unwrap();

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!("{}", "rukefile not found".color(Colors::RedFg));
            return;
        }
    };

    let rukefile = Rukefile::new(filepath);

    if let Err(e) = rukefile {
        eprintln!("{:?}", e);
        return;
    }

    let rukefile = rukefile.unwrap();

    rukefile.run_recipe(target.to_string(), *quiet)
}
