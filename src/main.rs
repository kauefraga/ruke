mod cli;

fn main() {
    let matches = cli::get_matches();

    let target = matches.get_one::<String>("target");
    let rukefile = matches.get_one::<String>("file");
    let quiet = matches.get_one::<bool>("quiet");

    println!("{:?}", rukefile);
    println!("{:?}", target);
    println!("{:?}", quiet);
}
