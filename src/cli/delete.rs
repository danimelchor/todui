use super::{cli_utils, formats::Format};
use crate::app::App;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// The ID of the task to delete
    #[arg(short, long)]
    id: usize,
    /// The format to print the deleted task with
    #[arg(short, long)]
    format: Option<Format>,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args { id, format } = args;
    let task = app.get_task(id).cloned();
    if task.is_none() {
        println!("Task with id {} not found", id);
    }

    let task_id = app.delete_task(id);

    match task_id {
        Some(_) => {
            cli_utils::print_task(&task.unwrap(), format, &app.settings);
        }
        None => println!("Task with id {} not found", id),
    }

    Ok(())
}
