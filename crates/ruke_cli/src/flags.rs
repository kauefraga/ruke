pub struct Flag {
    pub name: String,
    pub value: Option<String>
}

pub fn get_flags(arguments: &Vec<String>) -> Option<Vec<Flag>> {
    let mut flags = Vec::<Flag>::with_capacity(arguments.len());
    let mut arguments = arguments
        .iter()
        .peekable();

    while let Some(argument) = arguments.next() {
        if argument.contains("-") {
            if let Some(value) = arguments.peek() {
                if value.contains("-") {
                    flags.push(Flag{
                        name: argument.replace("-", ""),
                        value: None
                    });

                    continue;
                }

                flags.push(Flag{
                    name: argument.replace("-", ""),
                    value: Some(value.to_string())
                });

                continue;
            }

            flags.push(Flag{
                name: argument.replace("-", ""),
                value: None
            });
        }
    }

    if flags.len() == 0 {
        return None;
    }

    Some(flags)
}

pub fn show_help() {
    println!("A dead-simple automation tool.\n");

    println!("Usage:");
    println!("  ruke [target] [flags]\n");

    println!("Examples:");
    println!(
        "  {}\n  {}\n  {}\n  {}\n",
        "ruke",
        "ruke build",
        "ruke release --quiet",
        "ruke dev --file path/to/rufile"
    );

    println!("Flags:");
    println!(
        "  {}\n  {}\n  {}\n  {}",
        "-h, --help        help for ruke",
        "-v, --version     version for ruke",
        "-q, --quiet       execute target silently (no output)",
        "-f, --file FILE   read FILE as rufile"
    );
}

pub fn show_version() {
    println!("Ruke 0.1.0");
}
