use chrono::{Local, NaiveDate};

use crate::app::App;
use crate::task::Task;
use std::fs;
use std::io::prelude::*;

pub fn load_tasks(file: &str) -> Vec<Task> {
    let file = fs::read_to_string(file).expect("Unable to read file");
    let mut tasks: Vec<Task> = serde_json::from_str(&file).expect("Unable to parse file");
    tasks.sort_by(|a, b| a.date.cmp(&b.date));
    tasks
}

pub fn save_tasks(file: &str, app: &App) {
    let file = fs::File::create(file).expect("Unable to create file");
    serde_json::to_writer(file, &app.tasks).expect("Unable to write file");
}

pub fn date_to_str(dt: &NaiveDate) -> String {
    let today = Local::now().naive_local().date();
    let delta = dt.signed_duration_since(today);

    match delta.num_days() {
        0 => "Today".to_string(),
        1 => "Tomorrow".to_string(),
        2..=6 => dt.format("%A").to_string(),
        _ => dt.format("%Y-%m-%d").to_string(),
    }
}

pub fn log(s: String) {
    // Save to ~/.config/rust-todo/log.txt
    let mut open = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("/Users/danielmelchor/.config/rust-todo/log.txt")
        .expect("Unable to open log");

    writeln!(open, "{}", s).expect("Unable to write to log");
}
