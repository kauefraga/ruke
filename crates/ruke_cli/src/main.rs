use ruke_core;

fn main() {
    let args = ruke_core::get_arguments();

    if let None = args.get(0) {
        println!("main...");
        return;
    }

    match args[0].as_str() {
        "--help" | "-h" => ruke_core::show_help(),
        "--version" | "-v" => ruke_core::show_version(),
        arg => {
            println!("{}", arg);
            return;
        },
    }
}
