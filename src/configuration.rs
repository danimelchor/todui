use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub db_file: String,
    pub date_format: String,
    pub show_completed: bool,
    pub icons: Icons,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Icons {
    completed: String,
    incomplete: String,
}

impl Icons {
    pub fn default() -> Self {
        Icons {
            completed: "󰄴".to_string(),
            incomplete: "󰝦".to_string(),
        }
    }

    pub fn get_icon(&self, completed: bool) -> String {
        let icon = if completed {
            self.completed.clone()
        } else {
            self.incomplete.clone()
        };

        // Needs some padding
        format!(" {}", icon)
    }
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            db_file: Self::get_default_db_file(),
            date_format: "%Y-%m-%d".to_string(),
            show_completed: true,
            icons: Icons::default(),
        }
    }

    pub fn default_path() -> String {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/.config/rust-todo/", home);
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
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/.config/rust-todo/settings.json", home);
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

    pub fn set_show_completed(&mut self, show_completed: bool) {
        self.show_completed = show_completed;
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
