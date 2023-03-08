use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::app::App;
use crate::task::Task;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Ls {
        #[arg(long)]
        format: Option<Format>,
        #[arg(long)]
        filter: Option<Filter>,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    PlainText,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Filter {
    All,
    Today,
}

fn ls(app: &App, format: Option<Format>, filter: Option<Filter>) -> Result<()> {
    let mut tasks: Vec<&Task> = app.tasks.iter().filter(|t| !t.completed).collect();

    match filter {
        Some(Filter::Today) => {
            let today = chrono::Local::now().date_naive();
            tasks = tasks.iter().cloned().filter(|t| t.date == today).collect();
        }
        _ => {}
    }

    match format {
        Some(Format::Json) => {
            let json = serde_json::to_string(&tasks)?;
            println!("{}", json);
        }
        _ => {
            for task in tasks {
                println!("{}", task);
            }
        }
    }

    Ok(())
}

pub fn start_cli(app: &mut App) -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Command::Ls { format, filter } => ls(app, *format, *filter),
    }
}
