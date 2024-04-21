pub mod list;

use clap::{arg, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
    let cli = Command::new("ruke")
        .author("Kauê Fraga Rodrigues")
        .version("0.1.0")
        .about("A dead-simple automation tool. Inspired by Makefile and Justfile.")
        .arg(arg!([target] "Set the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Set run to be silent"))
        .subcommand(list::list_command());

    cli.get_matches()
}
