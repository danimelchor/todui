use super::{cli_utils, formats::Format};
use crate::app::App;
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    /// The ID of the task to modify
    #[arg(short, long)]
    id: usize,
    /// Whether the task should be marked as complete or incomplete
    #[arg(short, long)]
    complete: CompleteStatus,
    /// The format to print the updated task with
    #[arg(short, long)]
    format: Option<Format>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CompleteStatus {
    Complete,
    Incomplete,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        id,
        complete,
        format,
    } = args;
    let complete_bool = match complete {
        CompleteStatus::Complete => true,
        CompleteStatus::Incomplete => false,
    };
    let task = app.set_complete(id, complete_bool);

    if task.is_none() {
        println!("Task with id {} not found", id);
    }

    let task = task.unwrap();
    cli_utils::print_task(&task, format, &app.settings);

    Ok(())
}
