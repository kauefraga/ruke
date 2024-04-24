use std::path::{Path, PathBuf};

pub fn resolve_path(path: Option<&String>) -> Option<PathBuf> {
    if let Some(path) = path {
        return Some(Path::new(path).to_path_buf());
    }

    let possible_root_paths = ["ruke.toml", "Ruke.toml", "rukefile", "Rukefile"];

    let path = possible_root_paths.iter().find(|path| {
        let path = Path::new(path);

        path.exists()
    });

    path.map(|path| Path::new(path).to_path_buf())
}
