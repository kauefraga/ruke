pub fn get_arguments() -> Option<Vec<String>> {
    let args = std::env::args().skip(1);

    if args.len() == 0 {
        return None;
    }

    Some(args.collect())
}
