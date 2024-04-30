mod cli;
mod tasks;

fn main() {
    let matches = cli::root_command().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => cli::init::init_handler(sub_matches),
        Some(("list", sub_matches)) => cli::list::list_handler(sub_matches),
        Some(("new", sub_matches)) => cli::new::new_handler(sub_matches),
        Some(("add", sub_matches)) => cli::add::add_handler(sub_matches),
        Some(("remove", sub_matches)) => cli::remove::remove_handler(sub_matches),
        None => cli::root_handler(matches),
        _ => unreachable!(),
    }
}
