use std::{
    process::{Command, Stdio},
    thread,
};

use clap::error::Result;
use colorized::{Color, Colors};

use super::Task;

pub fn run_task(task: Task, quiet: bool, parallel: bool) {
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
    match parallel {
        true => parallel_runner(commands, quiet),
        false => sync_runner(commands, quiet),
    }

    println!("{}", "Task completed!".color(Colors::GreenFg));
}
fn parallel_runner(commands: Vec<String>, quiet: bool) {
    let mut threads = vec![];

    for command in commands.clone() {
        let sanitized_command = command
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        println!(
            "{} {}",
            "$".color(Colors::MagentaFg),
            sanitized_command.join(" ").color(Colors::BrightBlackFg)
        );

        let arguments = sanitized_command[1..]
            .iter()
            .map(|argument| argument.to_string())
            .collect::<Vec<String>>();

        threads.push(thread::spawn(
            move || -> Result<std::process::Output, std::io::Error> {
                Command::new(sanitized_command[0].clone())
                    .args(arguments)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
            },
        ));
    }

    for (idx, executed_commands) in threads.into_iter().enumerate() {
        match executed_commands.join() {
            Ok(Ok(output)) => {
                if output.status.success() {
                    if let Ok(stdout) = handle_output(Ok(output), quiet) {
                        if !stdout.is_empty() {
                            println!("Command {} output:\n{}\n", idx, stdout.trim_end());
                        }
                    }
                } else {
                    eprintln!(
                        "{}",
                        format!("Command {} failed.", commands[idx]).color(Colors::RedFg)
                    );
                }
            }
            Ok(Err(_)) => {
                eprintln!(
                    "{}",
                    format!("Failed to execute command {}", commands[idx]).color(Colors::RedFg)
                );
            }
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("Thread at command {} panicked: {:?}", commands[idx], e)
                        .color(Colors::RedFg)
                );
            }
        }
    }
}

fn sync_runner(commands: Vec<String>, quiet: bool) {
    for command in commands {
        let command = command.split(' ').collect::<Vec<&str>>();

        println!(
            "{} {}",
            "$".color(Colors::MagentaFg),
            command.join(" ").color(Colors::BrightBlackFg)
        );

        let arguments = command[1..]
            .iter()
            .map(|argument| argument.to_string())
            .collect::<Vec<String>>();

        let output = Command::new(command[0])
            .args(arguments)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output();

        if output.is_err() {
            eprintln!(
                "{}",
                format!("Failed executing the command `{}`.", command.join(" "))
                    .color(Colors::RedFg)
            );
            return;
        }

        let result = handle_output(output, quiet);

        let stdout = match result {
            Ok(stdout) => stdout,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        if stdout.is_empty() {
            continue;
        }

        println!("{}\n", stdout.trim_end());
    }
}

fn handle_output(
    output: Result<std::process::Output, std::io::Error>,
    quiet: bool,
) -> Result<String, String> {
    let output = output.unwrap();

    if quiet || !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(stderr.trim_end().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    return Ok(stdout);
}
