# Ruke

![GitHub top language](https://img.shields.io/github/languages/top/kauefraga/ruke)
![Crates.io Version](https://img.shields.io/crates/v/ruke)
![GitHub's license](https://img.shields.io/github/license/kauefraga/ruke)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kauefraga/ruke/main)

> A dead-simple automation tool. Inspired by Makefile and Justfile.

> [!TIP]
> Looking for a Dockerfile/docker-compose.yml generator? [gorvus](https://github.com/FelipeMCassiano/gorvus) is waiting for you!

## 🔑 Key Features

- Fancy interface: good experience, clear instructions and colored texts are what you get.
- Lightning speed: written in Rust, when you run it, it looks like a rocket.
- Easy configuration: unlike other command runners, Ruke uses TOML so you don't need to learn a new language.

## 🛠 Usage

**Pre-requisites**: Rust and Cargo.

### Installation

```bash
cargo install ruke
```

### Getting Started

First things first, you need to define your tasks in a `Ruke.toml` file.

Hopefully, Ruke can help you, just run

```bash
ruke init
```

With your `Ruke.toml` ready, now you need to run a specific task.

The syntax for running a task is `ruke [target]` where target is the task you wanna execute. Try this:

```bash
ruke
```

**Obs**.: the target task "main" is the default, therefore if you run `ruke`, it's the same as running `ruke main`.

### Mastering the CLI

###### Available commands

- `ruke init` - Create a `Ruke.toml` file with a task within
- `ruke list` - List the name of existing tasks
- `ruke add` - Add a new task
- `ruke remove` - Remove an existing task
- `ruke [target]` - Run a specific task

###### Aliases

- `ruke init`, `ruke i`
- `ruke list`, `ruke ls`
- `ruke add`, `ruke a`
- `ruke remove`, `ruke rm`

###### Arguments and flags

`ruke init` doesn't have arguments or flags.

`ruke list` has the flags `-a --all` and `-f --file <FILE>`.

`ruke add` has the flags `-n --name <NAME>`, `-c --command <COMMAND>` and `-f --file <FILE>`.

`ruke remove` has the flags `-n --name <NAME>` and `-f --file <FILE>`.

`ruke` has the optional argument `[target]` and the flags `-q --quiet` and `-f --file <FILE>`.

If you run `ruke --help` you'll see nice guide, and if you want help for a specific command, try `ruke help [command]`.

### Mastering the Rukefile

I suggest you to use one of these two names: `Ruke.toml` or `Rukefile`, however, as long as you write a valid TOML, you can name the file whatever you want and pass it with the `-f --file <FILE>` flag.

Look at [the full spec of TOML v1.0.0](https://toml.io/en/v1.0.0).

```toml
[[tasks]]                       # defines a task
name = "main"                   # defines an unique name to the task
command = "go run cmd/main.go"  # defines a command to be executed

[[tasks]]                       # defines other task
name = "dev"
command = "pnpm dev"
arguments = ["--watch"]         # specifies arguments to the command

[[tasks]]                       # another one
name = "build"
command = "go build -o gorvus cmd/main.go"
```

## 💖 Contributing

Feel free to contribute, create an issue to report a bug, suggest an API change, an improvement or a feature.

### How to contribute

1. Fork this repository
2. Clone your fork on your machine
3. Make your changes, commit and push these
4. Open a pull request (write a descriptive message about what you changed)

## 📝 License

This project is licensed under the MIT License - See the [LICENSE](https://github.com/kauefraga/ruke/blob/main/LICENSE) for more information.

---

Made with ❤ and 🦀 by Kauê Fraga Rodrigues.
