pub mod path;
pub mod runner;

use core::fmt;
use std::{fs, path::PathBuf};
use toml::ser::Error;

use colorized::{Color, Colors};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub commands: Option<Vec<String>>,
    pub arguments: Option<Vec<String>>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = match &self.arguments {
            Some(arguments) => arguments.join(", "),
            None => String::from("not defined").color(Colors::YellowFg),
        };

        let commands = match &self.commands {
            Some(commands) => commands.join(", "),
            None => String::from("not defined").color(Colors::YellowFg),
        };

        write!(
            f,
            "> {}\ncommands: {}\narguments: {}\n",
            self.name.color(Colors::GreenFg),
            commands.color(Colors::GreenFg),
            arguments.color(Colors::GreenFg)
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Task>,
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
        let task = self.tasks.iter().find(|task| task.name.eq(&name));

        task.cloned()
    }

    pub fn create_task(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("The task name must not be empty.".to_string());
        }

        let task = self.find_task(name.clone());

        if task.is_some() {
            return Err("A task with the same name already exists.".to_string());
        }

        let task = Task {
            name,
            commands: None,
            arguments: None,
        };

        self.tasks.push(task);
        Ok(())
    }

    pub fn add_command(&mut self, name: String, command: String) -> Result<(), String> {
        let task = self.find_task(name.clone());
        let task_index = self
            .tasks
            .iter()
            .enumerate()
            .find(|(_, task)| task.name.eq(&name))
            .map(|(index, _)| index);

        match task {
            Some(task) => {
                let commands = match task.commands {
                    Some(mut commands) => {
                        commands.push(command);
                        Some(commands)
                    }
                    None => Some(vec![command]),
                };

                let task = Task {
                    name,
                    commands,
                    arguments: None,
                };

                let task_index = task_index.unwrap();

                self.tasks.remove(task_index);
                self.tasks.push(task);

                Ok(())
            }
            None => Err("The task does not exist.".to_string()),
        }
    }

    pub fn remove_task(&mut self, name: String) -> Result<(), String> {
        let old_len = self.tasks.len();
        self.tasks.retain(|task| task.name != name);
        let new_len = self.tasks.len();

        if old_len == new_len {
            return Err(format!(
                r#"Cannot remove "{}". This task doesn't exist."#,
                name
            ));
        }

        Ok(())
    }
}
