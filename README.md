# Ruke

![GitHub top language](https://img.shields.io/github/languages/top/kauefraga/ruke)
![Crates.io Version](https://img.shields.io/crates/v/ruke)
![GitHub's license](https://img.shields.io/github/license/kauefraga/ruke)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kauefraga/ruke/main)

> A dead-simple automation tool. Inspired by Makefile and Justfile.

## 🔑 Key Features

- Fancy interface: good experience, clear instructions and colored texts are what you get.
- Lightning speed: written in Rust, when you run it, it looks like a rocket.
- Easy to use: unlike other command runners, Ruke uses TOML so you don't need to learn a new configuration language.

## 🛠 Usage

**Pre-requisites**: Rust and Cargo.

### Installation

Using Cargo, run:

```bash
cargo install ruke
```

### Getting Started

First things first, define your tasks in a `Ruke.toml`.

**Recommended**: place it in the root of your project.

```toml
[[tasks]]
name = "main"
command = "pnpm start"
arguments = ["--port 3333"]
```

Now, to run the `main` recipe

```bash
ruke
```

Yeah, it's that simple!

### Mastering the CLI

Here's two tables that show existing arguments, flags and their default value.

| Argument | Default |
|----------|---------|
| target   | main    |

| Flag           | Default   |
|----------------|-----------|
| `-f` `--file`  | Ruke.toml |
| `-q` `--quiet` | false     |

#### Examples

Root `Ruke.toml` and default target (named main)

```bash
ruke
```

Root `Ruke.toml` and non-default target

```bash
ruke target
```

Non-root `Ruke.toml`, silent and non-default target

```bash
ruke target --file path/to/Ruke.toml --quiet
```

### Mastering the Rukefile

As long as you write a valid TOML, you can name the file whatever you want. That being said, I suggest you to use one of these two names: `Ruke.toml` or `Rukefile`.

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
