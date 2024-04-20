use clap::{arg, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
    let cli = Command::new("ruke")
        .author("Kauê Fraga Rodrigues")
        .version("0.1.0")
        .about("A dead-simple automation tool. Inspired by Makefile and Justfile.")
        .arg(arg!([target] "Sets the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Sets a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Sets run to be silent"));

    cli.get_matches()
}
