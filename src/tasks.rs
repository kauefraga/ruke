use core::fmt;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub command: String,
    pub arguments: Option<Vec<String>>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = match &self.arguments {
            Some(args) => args.join(", "),
            None => String::from("not defined"),
        };

        write!(
            f,
            "  name: {}\n  command: {}\n  arguments: {}\n",
            self.name, self.command, arguments
        )
    }
}

#[derive(Clone, Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Recipe>,
}

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

#[derive(Debug)]
pub enum RukefileError {
    IoError(io::Error),
    TomlError(toml::de::Error),
}

impl Rukefile {
    pub fn new(path: PathBuf) -> Result<Self, RukefileError> {
        let raw_rukefile = fs::read_to_string(path);

        if let Err(e) = raw_rukefile {
            return Err(RukefileError::IoError(e));
        }

        match raw_rukefile {
            Ok(raw_rukefile) => {
                let rukefile = toml::from_str::<Rukefile>(&raw_rukefile);

                match rukefile {
                    Ok(rukefile) => Ok(rukefile),
                    Err(e) => Err(RukefileError::TomlError(e)),
                }
            }
            Err(e) => Err(RukefileError::IoError(e)),
        }
    }

    fn find_recipe(&self, name: String) -> Option<Recipe> {
        let recipe = self.tasks.iter().find(|recipe| recipe.name.eq(&name));

        recipe.cloned()
    }

    pub fn run_recipe(&self, name: String, quiet: bool) {
        let recipe = match self.find_recipe(name) {
            Some(recipe) => recipe,
            None => {
                eprintln!("recipe not found");
                return;
            }
        };

        let command = recipe.command.split(' ').collect::<Vec<&str>>();

        let positional_arguments = command[1..].iter().map(|argument| argument.to_string());

        let arguments = match recipe.arguments {
            Some(mut arguments) => {
                positional_arguments.for_each(|argument| arguments.push(argument));

                arguments
            }
            None => positional_arguments.collect::<Vec<String>>(),
        };

        let output = Command::new(command[0])
            .args(arguments)
            .output()
            .expect("failed to execute command");

        if output.status.success() && !quiet {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", stderr);
        }
    }

    pub fn list_tasks(&self) {
        println!("Tasks in recipe:");
        for t in self.tasks.iter() {
            println!("  {}", t.name);
        }
    }

    pub fn all_tasks(&self) {
        println!("All tasks in recipe:");

        for t in self.tasks.iter() {
            println!("{}", t);
        }
    }
}
