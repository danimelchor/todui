use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub db_file: String,
    pub date_formats: DateFormats,
    pub show_complete: bool,
    pub icons: Icons,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Icons {
    pub complete: String,
    pub incomplete: String,
    pub repeats: String,
}

impl Icons {
    pub fn default() -> Self {
        Icons {
            complete: "󰄴".to_string(),
            incomplete: "󰝦".to_string(),
            repeats: "".to_string(),
        }
    }

    pub fn get_complete_icon(&self, complete: bool) -> String {
        let icon = if complete {
            self.complete.clone()
        } else {
            self.incomplete.clone()
        };

        // Needs some padding
        format!(" {}", icon)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DateFormats {
    pub display_date_format: String,
    pub display_datetime_format: String,
    pub input_date_format: String,
    pub input_date_hint: String,
    pub input_datetime_format: String,
    pub input_datetime_hint: String,
}

impl DateFormats {
    fn new() -> Self {
        DateFormats {
            display_date_format: "%a %b %-d".to_string(),
            display_datetime_format: "%a %b %-d at %-H:%M".to_string(),
            input_datetime_format: "%d-%m-%Y %H:%M".to_string(),
            input_datetime_hint: "DD-MM-YYYY HH:MM".to_string(),
            input_date_format: "%d-%m-%Y".to_string(),
            input_date_hint: "DD-MM-YYYY".to_string(),
        }
    }
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            db_file: Self::get_default_db_file(),
            show_complete: true,
            icons: Icons::default(),
            date_formats: DateFormats::new(),
        }
    }

    pub fn default_path() -> String {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/.config/todo-rs", home);
        fs::create_dir_all(&path).unwrap();
        path
    }

    pub fn get_default_db_file() -> String {
        let path = Self::default_path();
        let path = format!("{}/tasks.json", path);

        if !std::path::Path::new(&path).exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .expect("Could not create db file");
            writeln!(file, "[]").unwrap();
        }

        path
    }

    pub fn get_settings_path() -> String {
        let path = Self::default_path();
        let path = format!("{}/settings.json", path);
        if !std::path::Path::new(&path).exists() {
            let settings = Settings::default();
            let settings_json = serde_json::to_string_pretty(&settings).unwrap();
            fs::write(&path, settings_json).unwrap();
        }
        path
    }

    pub fn save_state(&self) {
        let settings_path = Settings::get_settings_path();
        utils::save_settings(&settings_path, self);
    }

    pub fn set_show_complete(&mut self, show_complete: bool) {
        self.show_complete = show_complete;
        self.save_state();
    }
}

pub fn get_configuration() -> Settings {
    let settings_path = Settings::get_settings_path();
    config::Config::builder()
        .add_source(config::File::with_name(settings_path.as_str()))
        .build()
        .unwrap()
        .try_deserialize()
        .expect("Could not deserialize configuration")
}
