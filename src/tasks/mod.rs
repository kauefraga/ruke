pub mod path;

use core::fmt;
use std::{fs, io, path::PathBuf, process::Command};
use toml::ser::Error;

use colorized::{Color, Colors};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub command: String,
    pub arguments: Option<Vec<String>>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = match &self.arguments {
            Some(args) => args.join(", "),
            None => String::from("not defined").color(Colors::YellowFg),
        };

        write!(
            f,
            "> {}\ncommand: {}\narguments: {}\n",
            self.name.color(Colors::GreenFg),
            self.command.color(Colors::GreenFg),
            arguments.color(Colors::GreenFg)
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Recipe>,
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

    pub fn update_rukefile(&self, filepath: PathBuf) -> Result<(), Error> {
        let serialized = toml::to_string(self)?;

        fs::write(filepath, serialized).unwrap();
        Ok(())
    }

    fn find_recipe(&self, name: String) -> Option<Recipe> {
        let recipe = self.tasks.iter().find(|recipe| recipe.name.eq(&name));

        recipe.cloned()
    }

    pub fn run_recipe(&self, name: String, quiet: bool) {
        let recipe = match self.find_recipe(name) {
            Some(recipe) => recipe,
            None => {
                eprintln!("{}", "recipe not found".color(Colors::RedFg));
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

        let is_success_and_not_quiet = output.status.success() && !quiet;

        if !is_success_and_not_quiet {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("{}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    }

    pub fn list_tasks(&self) {
        for t in self.tasks.iter() {
            println!("{}", t.name.color(Colors::GreenFg));
        }
    }

    pub fn all_tasks(&self) {
        for t in self.tasks.iter() {
            println!("{}", t);
        }
    }

    pub fn add_task(&mut self, name: String, command: String) -> Result<(), &'static str> {
        for in_tasks in &self.tasks {
            if in_tasks.name == name {
                return Err("conflicting with a task with same name");
            }
        }

        let task = Recipe {
            name,
            command,
            arguments: None,
        };
        self.tasks.push(task);
        Ok(())
    }
}
