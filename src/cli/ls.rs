use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::app::App;
use crate::task::Task;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    format: Option<Format>,
    #[arg(short,long)]
    show_completed: bool,
    #[arg(long)]
    filter: Option<Filter>,
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

pub fn run(app: App, args: Args) -> Result<()> {
    let Args { format, show_completed, filter } = args;

    let mut tasks_iter: Box<dyn Iterator<Item = &Task>> = if !show_completed {
        Box::new(app.tasks.iter().filter(|&t| !t.completed))
    } else {
        Box::new(app.tasks.iter())
    };

    match filter {
        Some(Filter::Today) => {
            tasks_iter = Box::new(tasks_iter.filter(|&t| {
                let today = chrono::Local::now().date_naive();
                t.date == today
            }));
        }
        _ => {}
    }

    let tasks: Vec<&Task> = tasks_iter.collect();

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
