# todo-rs

## How to use?

You can run the TUI by executing `rust-todo` anywhere in your terminal. To use the CLI, you can start by running `rust-todo --help`:

```
$ rust-todo --help
A simple CLI and TUI for your todos

Usage: rust-todo <COMMAND>

Commands:
  ls        Lists all the tasks
  add       Adds a task to your todos
  delete    Deletes a task from your todos
  complete  Marks a task as complete or incomplete
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Config

The config file can be found in:
- Unix: `~/.config/rust-todo/settings.json`
- Windows: `C:\Users\<user>\AppData\Roaming\rust-todo\settings.json`

## Installation

Use rusts package manger to install todo-rs.

```
cargo install --git https://github.com/danimelchor/todo-rs.git
```
