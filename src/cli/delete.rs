use super::{cli_utils, formats::Format};
use crate::app::App;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    id: usize,
    #[arg(short, long)]
    format: Option<Format>,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args { id, format } = args;
    let task = app.delete_task(id);

    if task.is_none() {
        println!("Task with id {} not found", id);
    }

    let task = task.unwrap();
    cli_utils::print_task(&task, format, &app.settings);

    Ok(())
}
