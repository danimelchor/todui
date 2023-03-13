use anyhow::Result;
use clap::Parser;

use crate::app::App;
use crate::cli::cli_utils;
use crate::cli::formats::Format;
use crate::task_form::TaskForm;

#[derive(Parser)]
pub struct Args {
    name: String,
    #[arg(long)]
    date: Option<String>,
    #[arg(long)]
    repeats: Option<String>,
    #[arg(long)]
    description: Option<String>,
    #[arg(long)]
    format: Option<Format>,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args {
        name,
        format,
        date,
        repeats,
        description,
    } = args;
    let mut task_form = TaskForm {
        name,
        date: date.unwrap_or("".to_string()),
        repeats: repeats.unwrap_or("".to_string()),
        description: description.unwrap_or("".to_string()),
    };
    let task = task_form.submit()?;
    let task = app.add_task(task);

    cli_utils::print_task(&task, format, &app.settings);

    Ok(())
}
