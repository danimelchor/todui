use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::app::App;
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    PlainText,
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

    match format {
        Some(Format::Json) => {
            let json = serde_json::to_string(&task)?;
            println!("{}", json);
        }
        _ => {
            println!("{}", task);
        }
    }

    Ok(())
}
