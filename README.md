# todo-rs

## TUI

![Demo](https://github.com/danimelchor/todo-rs/tree/main/assets/demo.mov)

## How to use?

You can run the TUI by executing `rust-todo` anywhere in your terminal. To use the CLI, you can start by running `rust-todo --help`:

```
$ todo-rs --help
A CLI and TUI for your todos

Usage: todo-rs <COMMAND>

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
- Unix: `~/.config/todo-rs/settings.json`
- Windows: `C:\Users\<user>\AppData\Roaming\todo-rs\settings.json`

The default config is the following:

```json
{
  "db_file": "/Users/<USER>/.config/todo-rs/tasks.json",
  "date_formats": {
    "display_date_format": "%a %b %-d",
    "display_datetime_format": "%a %b %-d at %-H:%M",
    "input_date_format": "%d-%m-%Y",
    "input_date_hint": "DD-MM-YYYY",
    "input_datetime_format": "%d-%m-%Y %H:%M",
    "input_datetime_hint": "DD-MM-YYYY HH:MM"
  },
  "show_complete": true,
  "icons": {
    "complete": "󰄴",
    "incomplete": "󰝦",
    "repeats": ""
  },
  "colors": {
    "primary_color": "LightGreen",
    "secondary_color": "LightYellow",
    "accent_color": "LightBlue"
  }
}
```

## Installation

Use rusts package manger to install todo-rs.

```
cargo install --git https://github.com/danimelchor/todo-rs.git
```
