mod args;
mod flags;

fn main() {
    let args = args::get_arguments();

    match args {
        Some(args) => {
            match args[0].as_str() {
                "--help" | "-h" => flags::show_help(),
                "--version" | "-v" => flags::show_version(),
                arg => println!("target: {}", arg)
            }
        },
        None => {
            println!("executando target main...");
        }
    }
}
