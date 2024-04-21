use clap::{arg, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
    let cli = Command::new("ruke")
        .author("Kauê Fraga Rodrigues")
        .version("0.1.0")
        .about("A dead-simple automation tool. Inspired by Makefile and Justfile.")
        .arg(arg!([target] "Sets the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Sets a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Sets run to be silent"))
        .subcommand(create_ruke_list_cmd());

    cli.get_matches()
}

fn create_ruke_list_cmd() -> Command {
    Command::new("list")
        .about("List tasks in recipe")
        .arg(arg!(-a --all "List all tasks"))
        .alias("ls")
}
