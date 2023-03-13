# todo-rs

## How to use?

You can run the TUI by executing `rust-todo` anywhere in your terminal. To use the CLI, you can start by running `rust-todo --help`:

```
$ rust-todo --help
Usage: rust-todo <COMMAND>

Commands:
  ls        
  add       
  delete    
  complete  
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
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
