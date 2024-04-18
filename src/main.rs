mod cli;
mod tasks;

use std::fs;

use tasks::get_tasks;

fn main() {
    let matches = cli::get_matches();

    let target = matches.get_one::<String>("target");
    let file = matches.get_one::<String>("file");
    let quiet = matches.get_one::<bool>("quiet");

    if let Some(file) = file {
        let rukefile = fs::read_to_string(file).expect("invalid path");

        println!("{:?}", get_tasks(rukefile));
    }
}
