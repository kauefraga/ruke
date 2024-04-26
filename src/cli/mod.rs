pub mod add;
pub mod init;
pub mod list;
pub mod remove;

use clap::{arg, ArgMatches, Command};
use colorized::{Color, Colors};

use crate::tasks::{path::resolve_path, Rukefile};

pub fn root_command() -> Command {
    Command::new("ruke")
        .author("Kauê Fraga Rodrigues")
        .version("0.1.3")
        .about("A dead-simple automation tool. Inspired by Makefile and Justfile.")
        .arg(arg!([target] "Set the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Set run to be silent"))
        .subcommand(init::init_command())
        .subcommand(list::list_command())
        .subcommand(add::add_command())
        .subcommand(remove::remove_command())
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
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    let rukefile = rukefile.unwrap();

    rukefile.run_task(target.to_string(), *quiet)
}
