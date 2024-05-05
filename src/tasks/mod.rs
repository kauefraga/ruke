pub mod path;
pub mod runner;

use colorized::{Color, Colors};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, path::PathBuf};
use toml::ser::Error;

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub commands: Option<Vec<String>>,
}

impl Task {
    pub fn display(&self, name: &str) -> String {
        let mut output = format!(
            "  {} ruke {}\n",
            "$".color(Colors::BrightBlackFg),
            name.color(Colors::BlueFg),
        );

        if self.commands.is_none() {
            output.push_str(&"    not defined\n".color(Colors::BrightBlackFg));
            return output;
        }

        let commands = self.commands.as_ref().unwrap();

        for command in commands {
            output.push_str(&format!("    {}\n", command.color(Colors::BrightBlackFg)));
        }

        output
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Rukefile {
    pub tasks: HashMap<String, Task>,
}

impl Rukefile {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let raw_rukefile = fs::read_to_string(path);

        match raw_rukefile {
            Ok(raw_rukefile) => {
                let rukefile = toml::from_str::<Rukefile>(&raw_rukefile);

                match rukefile {
                    Ok(rukefile) => Ok(rukefile),
                    Err(e) => Err(format!("Failed parsing TOML. Error: {}.", e)),
                }
            }
            Err(e) => Err(format!("Failed reading file. Error: {}.", e)),
        }
    }

    pub fn update_rukefile(&self, filepath: PathBuf) -> Result<(), Error> {
        let serialized = toml::to_string(self)?;

        fs::write(filepath, serialized).unwrap();

        Ok(())
    }

    pub fn find_task(&self, name: String) -> Option<Task> {
        self.tasks.get(&name).cloned()
    }

    pub fn create_task(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("The task name must not be empty.".to_string());
        }

        if self.tasks.contains_key(&name) {
            return Err("A task with the same name already exists.".to_string());
        }

        let task = Task { commands: None };

        self.tasks.insert(name, task);
        Ok(())
    }

    pub fn add_command(&mut self, name: String, command: String) -> Result<(), String> {
        match self.tasks.get_mut(&name) {
            Some(task) => {
                match &mut task.commands {
                    Some(commands) => commands.push(command),
                    None => task.commands = Some(vec![command]),
                };
                Ok(())
            }
            None => Err("The task does not exist.".to_string()),
        }
    }

    pub fn remove_task(&mut self, name: String) -> Result<(), String> {
        if self.tasks.remove(&name).is_none() {
            return Err(format!(
                r#"Cannot remove "{}". This task doesn't exist."#,
                name
            ));
        }

        Ok(())
    }
}
