mod cli;
mod tasks;

use tasks::{resolve_path, Rukefile};

fn main() {
    let matches = cli::get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    let filepath = matches.get_one::<String>("file");
    let quiet = matches.get_one::<bool>("quiet").unwrap();

    let filepath = match resolve_path(filepath) {
        Some(resolved_path) => resolved_path,
        None => {
            eprintln!("rukefile not found");
            return;
        }
    };

    let rukefile = Rukefile::new(filepath);

    match rukefile {
        Ok(rukefile) => {
            if let Some(list_matches) = matches.subcommand_matches("list") {
                if *list_matches.get_one::<bool>("all").unwrap_or(&false) {
                    rukefile.all_tasks();
                    return;
                }
                rukefile.list_tasks();
                return;
            }

            rukefile.run_recipe(target.to_string(), *quiet)
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}
