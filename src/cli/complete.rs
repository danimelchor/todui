use super::{cli_utils, formats::Format};
use crate::app::App;
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    id: usize,
    #[arg(short, long)]
    completed: CompletedStatus,
    #[arg(short, long)]
    format: Option<Format>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CompletedStatus {
    Complete,
    Incomplete,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        id,
        completed,
        format,
    } = args;
    let completed_bool = match completed {
        CompletedStatus::Complete => true,
        CompletedStatus::Incomplete => false,
    };
    let task = app.set_completed(id, completed_bool);

    if task.is_none() {
        println!("Task with id {} not found", id);
    }

    let task = task.unwrap();
    cli_utils::print_task(&task, format, &app.settings);

    Ok(())
}
