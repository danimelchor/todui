use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::app::App;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    id: usize,
    #[arg(short, long)]
    format: Option<Format>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    PlainText,
}

pub fn run(mut app: App, args: Args) -> Result<()> {
    let Args { id, format } = args;
    let task = app.delete_task(id);

    if task.is_none() {
        println!("Task with id {} not found", id);
    }

    let task = task.unwrap();
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
