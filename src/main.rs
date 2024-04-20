mod cli;
mod tasks;

use std::fs;

use tasks::Rukefile;

fn main() {
    let matches = cli::get_matches();

    let target = matches.get_one::<String>("target").expect("required target");
    let file = matches.get_one::<String>("file").expect("required file");
    let _quiet = matches.get_one::<bool>("quiet");

    let rukefile = fs::read_to_string(file).expect("invalid path");
    let rukefile = Rukefile::from_str(&rukefile).expect("invalid toml");

    rukefile.tasks.iter()
        .for_each(|recipe| println!("{:?}", recipe));

    let recipe = rukefile.find_recipe(target.to_string());

    println!("{:?}", recipe);
}
