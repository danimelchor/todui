use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    pub db_file: String,
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            db_file: Self::get_default_db_file(),
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
