# Ruke

![GitHub top language](https://img.shields.io/github/languages/top/kauefraga/ruke)
![Crates.io Version](https://img.shields.io/crates/v/ruke)
![GitHub's license](https://img.shields.io/github/license/kauefraga/ruke)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/kauefraga/ruke/main)

> A dead-simple automation tool. Inspired by Makefile and Justfile.

## 🔑 Key Features

- Fancy interface: clear instructions and colored texts are what you get.
- Lightning speed: written in Rust, when you run it, it looks like a rocket.
- Easy to use: unlike other command runners, Ruke use TOML so you don't need to learn a new language.

## 🛠 Usage

**Pre-requisites**: Rust and Cargo.

### Installation

Using Cargo, run:

```bash
cargo install ruke
```

### Getting Started

First things first, define your scripts in a Ruke.toml

```toml
[[tasks]]
name = "main"
command = "pnpm dev"
args = "--port 3333"
env = ""
```

Now, to run the `main` script

```bash
ruke
```

Yeah, it is that simple!

## 🧱 Project Structure

```bash
— ...
— Cargo.toml - workspace config
— crates/
—— ruke_cli/ - interface (binary)
—— ruke_core/ - business logic (library)
```

## 💖 Contributing

Feel free to fork it, make a change and open a pull request. Same for issues, suggest an API change, an improvement, a feature or report a bug.

### How to contribute

1. Fork this repository
2. Clone your fork on your machine
3. Make your changes, commit and push these
4. Open a pull request (write a descriptive message about what you changed)

## 📝 License

This project is licensed under the MIT License - See the [LICENSE](https://github.com/kauefraga/ruke/blob/main/LICENSE) for more information.

---

Made with ❤ and 🦀 by Kauê Fraga Rodrigues.
