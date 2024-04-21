mod cli;
mod tasks;

use colorized::{Color, Colors};
use tasks::{resolve_path, Rukefile};

fn main() {
    let matches = cli::get_matches();

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

    let rukefile = rukefile.expect("Something went wrong :(");

    match matches.subcommand() {
        Some(("list", sub_matches)) => cli::list::list_handler(sub_matches, rukefile),
        None => rukefile.run_recipe(target.to_string(), *quiet),
        _ => unreachable!()
    }
}
