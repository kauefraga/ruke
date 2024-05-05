use std::process::Command;

use colorized::{Color, Colors};

use super::Task;

pub fn run_task(task: Task, quiet: bool) {
    let commands = match task.commands {
        Some(commands) => commands,
        None => {
            println!(
                "{}",
                "There are no commands to be run.".color(Colors::RedFg)
            );
            return;
        }
    };

    for command in commands {
        let command = command.split(' ').collect::<Vec<&str>>();

        let arguments = command[1..]
            .iter()
            .map(|argument| argument.to_string())
            .collect::<Vec<String>>();

        let output = Command::new(command[0])
            .args(arguments)
            .output()
            .expect("Failed to execute command");

        let is_success_and_not_quiet = output.status.success() && !quiet;

        match is_success_and_not_quiet {
            true => {
                println!(
                    "{} {}",
                    "$".color(Colors::MagentaFg),
                    command.join(" ").color(Colors::BrightBlackFg)
                );

                let stdout = String::from_utf8_lossy(&output.stdout);

                if stdout.is_empty() {
                    println!("{}", "Task completed!".color(Colors::GreenFg));
                    return;
                }

                println!("{}\n", stdout.trim_end());
            }
            false => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("{}", stderr.trim_end());
            }
        }
    }
}
