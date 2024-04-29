use std::{env, path::PathBuf};

const RUKE_FILE_NAMES: [&str; 4] = ["ruke.toml", "Ruke.toml", "rukefile", "Rukefile"];

fn resolve_file(root_path: PathBuf) -> Option<PathBuf> {
    for file_name in RUKE_FILE_NAMES {
        let mut root_path = root_path.clone();
        root_path.push(file_name);

        if root_path.exists() {
            return Some(root_path);
        }
    }

    None
}

pub fn resolve_path(path: Option<&String>) -> Option<PathBuf> {
    match path {
        Some(path) => Some(PathBuf::from(path)),
        None => {
            let current_directory = env::current_dir().ok()?;
            let path_ancestors = current_directory.ancestors();

            for path in path_ancestors {
                let path = path.to_path_buf();

                if let Some(resolved_file) = resolve_file(path) {
                    return Some(resolved_file);
                }
            }

            None
        }
    }
}
