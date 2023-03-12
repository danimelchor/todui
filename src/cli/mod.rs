use anyhow::Result;
use clap::Parser;
use crate::app::App;

mod ls;
mod add;
mod delete;
mod complete;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Ls(ls::Args),
    Add(add::Args),
    Delete(delete::Args),
    Complete(complete::Args),
}

pub fn start_cli(app: App) -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Ls(args) => ls::run(app, args),
        Command::Add(args) => add::run(app, args),
        Command::Delete(args) => delete::run(app, args),
        Command::Complete(args) => complete::run(app, args),
    }
}
