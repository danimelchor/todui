use std::collections::HashMap;

use crate::app::{App, Id};
use crate::configuration::Settings;
use crate::task::Task;
use crate::utils;
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
    /// Filter tasks by relative date
    #[arg(long)]
    date_filter: Option<DateFilter>,
    /// Filter tasks by date
    #[arg(long)]
    date: Option<String>,
    /// Filter by group
    #[arg(long)]
    group: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DateFilter {
    All,
    Today,
    Past,
    TodayAndPast,
    Next24,
}

pub fn filter_by_relative_date(
    tasks: HashMap<Id, Task>,
    date_filter: Option<DateFilter>,
) -> HashMap<Id, Task> {
    let now = chrono::Local::now();
    match date_filter {
        Some(DateFilter::Today) => tasks
            .into_iter()
            .filter(|(_, t)| {
                let today = now.date_naive();
                t.date.date_naive() == today
            })
            .collect(),
        Some(DateFilter::Past) => tasks.into_iter().filter(|(_, t)| t.date < now).collect(),
        Some(DateFilter::TodayAndPast) => tasks
            .into_iter()
            .filter(|(_, t)| {
                let today = now.date_naive();
                t.date.date_naive() <= today
            })
            .collect(),
        Some(DateFilter::Next24) => tasks
            .into_iter()
            .filter(|(_, t)| {
                let tomorrow = now + chrono::Duration::days(1);
                t.date >= now && t.date < tomorrow
            })
            .collect(),
        _ => tasks,
    }
}

pub fn filter_by_exact_date(
    tasks: HashMap<Id, Task>,
    date: Option<String>,
    settings: &Settings,
) -> Result<HashMap<Id, Task>> {
    let tasks = match date {
        Some(date) => {
            let date = utils::parse_date(date.as_str(), settings)?;
            tasks
                .into_iter()
                .filter(|(_, t)| &t.date == &date)
                .collect()
        }
        None => tasks,
    };
    Ok(tasks)
}

pub fn filter_by_group(tasks: HashMap<Id, Task>, group: Option<String>) -> HashMap<Id, Task> {
    match group {
        Some(group) => {
            let group = group.to_lowercase();
            tasks
            .into_iter()
            .filter(|(_, t)| t.group.as_ref().map(|g| g.to_lowercase() == group).unwrap_or(false))
            .collect()},
        None => tasks,
    }
}

pub fn run(app: App, args: Args) -> Result<()> {
    let Args {
        format,
        show_complete,
        show_descriptions,
        show_urls,
        date_filter,
        date,
        group,
    } = args;

    let tasks: HashMap<Id, Task> = if !show_complete {
        app.tasks.into_iter().filter(|(_, t)| !t.complete).collect()
    } else {
        app.tasks
    };

    let tasks = filter_by_relative_date(tasks, date_filter);
    let tasks = filter_by_exact_date(tasks, date, &app.settings)?;
    let tasks = filter_by_group(tasks, group);

    cli_utils::print_tasks(tasks, format, show_descriptions, show_urls, &app.settings);

    Ok(())
}
