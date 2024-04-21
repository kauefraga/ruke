mod cli;
mod tasks;

fn main() {
    let matches = cli::root_command().get_matches();

    match matches.subcommand() {
        Some(("list", sub_matches)) => cli::list::list_handler(sub_matches),
        None => cli::root_handler(matches),
        _ => unreachable!()
    }
}
