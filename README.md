# todo-rs

## TUI

https://user-images.githubusercontent.com/24496843/225213938-f2ade4b2-56c0-489c-ab5b-65c191b8c7bd.mp4

## Features

This app allows for almost anythig you would need when dealing with todos:
- Create, edit, and delete tasks
- Add links to tasks
- Add due dates to tasks
- Add repeating tasks
- Add notes to tasks
- Add tasks to groups (e.g. work, personal, etc.)

## How to use?

You can run the TUI by executing `todo-rs` anywhere in your terminal. To use the CLI, you can start by running `todo-rs --help`:

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

## Installation

Use rusts package manger to install todo-rs.

```
cargo install --git https://github.com/danimelchor/todo-rs.git
```

## Dependencies

This tool doesn't have any mandatory dependencies. However, it looks much better if you install [Nerd Fonts](https://www.nerdfonts.com/) for better icons. If you don't want to do so, you can always use your own icons or change them for plain text, like `[ ]` for an incomplete task and `[x]` for a complete task.

## [Documentation](https://github.com/danimelchor/todo-rs/blob/main/DOCUMENTATION.md)

## Config

The config file can be found in:
- Unix: `~/.config/todo-rs/settings.json`
- Windows: `C:\Users\<user>\AppData\Roaming\todo-rs\settings.json`

The default config is the following:

```json
{
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
  },
  "keybindings": {
    "quit": "q",
    "down": "j",
    "up": "k",
    "complete_task": "x",
    "toggle_completed_tasks": "c",
    "delete_task": "d",
    "new_task": "n",
    "edit_task": "e",
    "save_changes": "Enter",
    "enter_insert_mode": "i",
    "enter_normal_mode": "Esc",
    "go_back": "b",
    "open_link": "Enter",
    "next_group": "l",
    "prev_group": "h"
  }
}
```

For more options, head to [the documentation](https://github.com/danimelchor/todo-rs/blob/main/DOCUMENTATION.md)

## Key Bindings

All key bindings can be modified in the config file. The defaults have been chosen to mimic vim movements as best as possible. Feel free to modify them to your liking!

**List of tasks panel**

| Key Bindings | Description |
| -------- | ---------- | 
| `q` | Quits the application | 
| `j` | Moves down one task |
| `k` | Moves up one task |
| `x` | Marks the task as completed | 
| `c` | Toggles hiding completed tasks |
| `d` | Deletes the selected task forever|
| `Enter` | If the task has an associated link, it opens it in your preferred browser |
| `n` | Opens the new task page |
| `e` | Focuses the task editing panel |
| `l` | Select next group |
| `h` | Select previous group |

**Editing/new task panel**

This panel has two modes (similar to vim). When you are in insert mode, you can modify the fields to edit or create a task. When you are in normal mode, you can move around the fields, save the tasks, go back, or quit.

*Normal mode*

| Key Bindings | Description |
| -------- | ---------- |
| `j` | Move down to the next field |
| `k` | Move up to the previous field |
| `q` | Quit the application |
| `i` | Enter insert mode |
| `b` | Go back to the list of tasks panel |
| `Enter` | Save changes or add the new task |

*Insert mode*

| Key Bindings | Description |
| -------- | ---------- |
| `Esc` | Exit insert mode / go back to normal mode |

## Why the CLI?

CLI access to your todos introduces a programmatic way to modify or display your todos in comfortable places. For developers, this might mean displaying your todos when you open your terminal, as notifications, or even or your menu bar. For me, the menu bar was what drove me to create this project. I have used the app Cron for a bit and loved being able to see my events for that day without opening anything. So I created my own SketchyBar widget to interact with my todos:

https://user-images.githubusercontent.com/24496843/225197941-0dc04074-58d7-496e-a65d-692220fea809.mov

