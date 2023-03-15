# todo-rs

## TUI

https://user-images.githubusercontent.com/24496843/225117353-7b15a5cf-d313-47ed-ba30-58fc5fae6162.mov

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

## Keybinds

**List of tasks panel**

| Keybinds | Description |
| --------------------- | 
| q | Quits the application | 
| j | Moves down one task |
| k | Moves up one task |
| x | Marks the task as completed | 
| h | Toggles hiding completed tasks |
| d | Deletes the selected task forever|
| Enter | If the task has an associated link, it opens it in your preferred browser |
| n | Opens the new task page |
| e | Focuses the task editing panel |

**Editing/new task panel**

This panel has two modes (similar to vim). When you are in insert mode, you can modify the fields to edit or create a task. When you are in normal mode, you can move around the fields, save the tasks, go back, or quit.

*Normal mode*

| Keybinds | Description|
| --------------------- |
| j | Move down to the next field |
| k | Move up to the previous field |
| q | Quit the application |
| i | Enter insert mode |
| b | Go back to the list of tasks panel |
| Enter | Save changes or add the new task |

*Insert mode*

| Keybinds | Description|
| --------------------- |
| Esc | Exit insert mode / go back to normal mode |

## Installation

Use rusts package manger to install todo-rs.

```
cargo install --git https://github.com/danimelchor/todo-rs.git
```
