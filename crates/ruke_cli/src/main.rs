mod args;
mod flags;

fn main() {
    let arguments = args::get_arguments();

    match arguments {
        Some(arguments) => {
            let flags = flags::get_flags(&arguments);

            match flags {
                Some(flags) => {
                    for flag in flags {
                        // todo: shorthand -h
                        if flag.name.contains("help") {
                            flags::show_help();
                            continue;
                        }

                        // todo: shorthand -v
                        if flag.name.contains("version") {
                            flags::show_version();
                            continue;
                        }

                        println!(
                            "Name: {}, value: {:?}",
                            flag.name,
                            flag.value
                        )
                    }
                },
                None => println!("no flags")
            }
        },
        None => {
            println!("executando target main...");
        }
    }
}
