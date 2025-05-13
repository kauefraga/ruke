pub mod add;
pub mod init;
pub mod list;
pub mod new;
pub mod remove;

use clap::{arg, command, ArgMatches, Command};
use colorized::{Color, Colors};

use crate::tasks::{path::resolve_path, runner, Rukefile};

pub fn root_command() -> Command {
    command!()
        .arg(arg!([target] "Set the target task").default_value("main"))
        .arg(arg!(-f --file <FILE> "Set a Ruke.toml or Rukefile to use"))
        .arg(arg!(-q --quiet "Set run to be silent"))
        .arg(arg!(-p --parallel "Set the run mode to parallel"))
        .subcommand(add::add_command())
        .subcommand(init::init_command())
        .subcommand(list::list_command())
        .subcommand(new::new_command())
        .subcommand(remove::remove_command())
}

pub fn root_handler(matches: ArgMatches) {
    let target = matches.get_one::<String>("target").unwrap();
    let filepath = matches.get_one::<String>("file");
    let quiet = matches.get_one::<bool>("quiet").unwrap();
    let parallel = matches.get_one::<bool>("parallel").unwrap();

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!(
                "{}",
                "No ruke file found (looking for: Rukefile, rukefile, Ruke.toml, ruke.toml)."
                    .color(Colors::RedFg)
            );
            println!(
                "Try `{}` or `{}`.",
                "ruke init".color(Colors::BlueFg),
                "ruke -h".color(Colors::BlueFg)
            );
            return;
        }
    };

    let rukefile = Rukefile::new(filepath);

    if let Err(e) = rukefile {
        eprintln!("{}", e.color(Colors::RedFg));
        return;
    }

    let rukefile = rukefile.unwrap();

    let task = rukefile.find_task(target.clone());

    match task {
        Some(task) => runner::run_task(task, *quiet, *parallel),
        None => eprintln!(
            "{}",
            format!(r#"There is no "{}" task to run."#, target).color(Colors::RedFg)
        ),
    }
}
