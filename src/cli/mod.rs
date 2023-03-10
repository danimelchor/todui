use anyhow::Result;
use clap::Parser;
use crate::app::App;

mod ls;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Ls(ls::Args),
}

pub fn start_cli(app: App) -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Ls(args) => ls::run(app, args),
    }
}
