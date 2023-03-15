use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDate, TimeZone, Timelike};

use crate::app::App;
use crate::configuration::Settings;
use crate::task::Task;
use std::fs;
use std::path::PathBuf;

pub fn load_tasks(file: PathBuf) -> Vec<Task> {
    let file = fs::read_to_string(file).expect("Unable to read file");
    let mut tasks: Vec<Task> = serde_json::from_str(&file).expect("Unable to parse file");
    tasks.sort_by(|a, b| a.date.cmp(&b.date));
    tasks
}

pub fn save_tasks(file: PathBuf, app: &App) {
    let file = fs::File::create(file).expect("Unable to create file");
    serde_json::to_writer(file, &app.tasks).expect("Unable to write file");
}

pub fn save_settings(file: &PathBuf, settings: &Settings) {
    let file = fs::File::create(file).expect("Unable to create file");
    serde_json::to_writer(file, &settings).expect("Unable to write file");
}

pub fn date_has_time(date: &DateTime<Local>) -> bool {
    let time = date.time();
    if time.hour() == 23 && time.minute() == 59 {
        return false;
    }
    true
}

pub fn date_to_display_str(dt: &DateTime<Local>, settings: &Settings) -> String {
    let format;
    if date_has_time(&dt) {
        format = settings.date_formats.display_datetime_format.clone();
    } else {
        format = settings.date_formats.display_date_format.clone();
    }
    dt.format(format.as_str()).to_string()
}

pub fn date_to_input_str(dt: &DateTime<Local>, settings: &Settings) -> String {
    let format;
    if date_has_time(&dt) {
        format = settings.date_formats.input_datetime_format.clone();
    } else {
        format = settings.date_formats.input_date_format.clone();
    }
    dt.format(format.as_str()).to_string()
}

pub fn get_today() -> DateTime<Local> {
    let today = Local::now().date_naive().and_hms_opt(23, 59, 59).unwrap();
    Local.from_local_datetime(&today).unwrap()
}

pub fn parse_date(s: &String, settings: &Settings) -> Result<DateTime<Local>> {
    let datetime_format = settings.date_formats.input_datetime_format.as_str();
    let date_format = settings.date_formats.input_date_format.as_str();

    let attempt_datetime = Local.datetime_from_str(s.as_str(), datetime_format);
    let attempt_date = NaiveDate::parse_from_str(s.as_str(), date_format);

    if attempt_datetime.is_ok() {
        Ok(attempt_datetime.unwrap())
    } else if attempt_date.is_ok() {
        let date = attempt_date.unwrap();
        let datetime = date.and_hms_opt(23, 59, 59).unwrap();
        Ok(Local.from_local_datetime(&datetime).unwrap())
    } else {
        Err(anyhow!("Unable to parse date"))
    }
}
