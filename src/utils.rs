use chrono::{Local, NaiveDate};

use crate::app::App;
use crate::configuration::Settings;
use crate::task::Task;
use std::fs;

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

pub fn save_settings(file: &str, settings: &Settings) {
    let file = fs::File::create(file).expect("Unable to create file");
    serde_json::to_writer(file, &settings).expect("Unable to write file");
}

pub fn date_to_str(dt: &NaiveDate, format: &String) -> String {
    let today = Local::now().naive_local().date();
    let delta = dt.signed_duration_since(today);

    match delta.num_days() {
        0 => "Today".to_string(),
        1 => "Tomorrow".to_string(),
        2..=6 => dt.format("%A").to_string(),
        _ => dt.format(format.as_str()).to_string(),
    }
}
