mod cli;
mod tasks;

use std::fs;

use tasks::Rukefile;

fn main() {
    let matches = cli::get_matches();

    let _target = matches.get_one::<String>("target");
    let file = matches.get_one::<String>("file");
    let _quiet = matches.get_one::<bool>("quiet");

    if let Some(file) = file {
        let rukefile = fs::read_to_string(file).expect("invalid path");
        let rukefile = Rukefile::from_str(&rukefile);

        rukefile.unwrap().tasks.iter().for_each(|task| println!("{:?}", task));
    }
}
