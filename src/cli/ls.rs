use std::collections::HashMap;

use crate::app::{App, Id};
use crate::task::Task;
use anyhow::Result;
use clap::{Parser, ValueEnum};

use super::cli_utils;
use super::formats::Format;

#[derive(Parser)]
pub struct Args {
    /// The format to print the tasks with
    #[arg(long)]
    format: Option<Format>,
    /// Whether to show complete tasks
    #[arg(short, long)]
    show_complete: bool,
    /// Whether to show task descriptions
    #[arg(long)]
    show_descriptions: bool,
    /// Whether to show task urls
    #[arg(long)]
    show_urls: bool,
    /// Filter the tasks to show
    #[arg(long)]
    filter: Option<Filter>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Filter {
    All,
    Today,
    Past,
    TodayAndPast,
    Next24,
}

pub fn run(app: App, args: Args) -> Result<()> {
    let Args {
        format,
        show_complete,
        show_descriptions,
        show_urls,
        filter,
    } = args;

    let mut tasks_iter: Box<dyn Iterator<Item = (Id, Task)>> = if !show_complete {
        Box::new(app.tasks.into_iter().filter(|(_, t)| !t.complete))
    } else {
        Box::new(app.tasks.into_iter())
    };

    let now = chrono::Local::now();

    match filter {
        Some(Filter::Today) => {
            tasks_iter = Box::new(tasks_iter.filter(|(_, t)| {
                let today = now.date_naive();
                t.date.date_naive() == today
            }));
        }
        Some(Filter::Past) => {
            tasks_iter = Box::new(tasks_iter.filter(|(_, t)| t.date < now));
        }
        Some(Filter::TodayAndPast) => {
            tasks_iter = Box::new(tasks_iter.filter(|(_, t)| {
                let today = now.date_naive();
                t.date.date_naive() <= today
            }));
        }
        Some(Filter::Next24) => {
            tasks_iter = Box::new(tasks_iter.filter(|(_, t)| {
                let tomorrow = now + chrono::Duration::days(1);
                t.date >= now && t.date < tomorrow
            }));
        }
        _ => {}
    }

    let tasks: HashMap<Id, Task> = tasks_iter.collect();
    cli_utils::print_tasks(tasks, format, show_descriptions, show_urls, &app.settings);

    Ok(())
}
