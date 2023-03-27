use anyhow::Result;
use clap::Parser;
use crate::app::App;

mod ls;
mod add;
mod delete;
mod complete;
mod config;
mod cli_utils;

// Shared enums and structs
mod formats;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    /// Lists all the tasks
    Ls(ls::Args),
    /// Adds a task to your todos
    Add(add::Args),
    /// Deletes a task from your todos
    Delete(delete::Args),
    /// Marks a task as complete or incomplete
    Complete(complete::Args),
    /// Sets default configurations
    Config(config::Args)
}

pub fn start_cli(app: App) -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Ls(args) => ls::run(app, args),
        Command::Add(args) => add::run(app, args),
        Command::Delete(args) => delete::run(app, args),
        Command::Complete(args) => complete::run(app, args),
        Command::Config(args) => config::run(app, args)
    }
}
