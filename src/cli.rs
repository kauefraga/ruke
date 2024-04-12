pub fn get_arguments() -> Vec<String> {
  let args = std::env::args().skip(1);

  args.collect()
}

pub fn show_help() {
  println!("A dead-simple automation tool. Inspired by GNU Make.\n");

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
