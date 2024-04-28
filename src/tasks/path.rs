use std::{env, path::PathBuf};

const RUKE_FILE_NAMES: [&str; 4] = ["ruke.toml", "Ruke.toml", "rukefile", "Rukefile"];

pub fn resolve_file<'a>(root_path: &PathBuf) -> Option<&'a &'a str> {
    RUKE_FILE_NAMES
        .iter()
        .find(|path| root_path.with_file_name(path).exists())
}

pub fn resolve_path(path: Option<&String>) -> Option<PathBuf> {
    match path {
        Some(path) => Some(PathBuf::from(path)),
        None => {
            let current_directory = env::current_dir().unwrap();
            let mut path_ancestors = current_directory.ancestors();

            while let Some(path) = path_ancestors.next() {
                let path = path.to_path_buf();

                if let Some(file_name) = resolve_file(&path) {
                    return Some(path.with_file_name(file_name));
                }
            }

            return None;
        }
    }
}
