mod cli;
mod rukefile;

use rukefile::{
    execute_task,
    get_rukefile
};

fn main() {
    let args = cli::get_arguments();

    if let None = args.get(0) {
        let rukefile = get_rukefile("");

        match rukefile {
            Ok(rukefile) => {
                println!("{}", rukefile);
                execute_task("main");
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        return;
    }

    match args[0].as_str() {
        "--help" | "-h" => cli::show_help(),
        "--version" | "-v" => cli::show_version(),
        arg => {
            let rukefile = get_rukefile("");

            match rukefile {
                Ok(rukefile) => {
                    println!("{}", rukefile);
                    execute_task(arg);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }

            return;
        },
    }
}
