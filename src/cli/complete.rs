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

    let task_id = app.set_complete(id, complete_bool);
    match task_id {
        Some(task_id) => {
            let task = app.get_task(task_id).unwrap();
            cli_utils::print_task(task_id, &task, format, &app.settings);
        }
        None => {
            println!("Task with id {} not found", id);
        }
    }

    Ok(())
}
