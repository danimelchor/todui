use crate::utils;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
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

pub fn serialize_color<S>(color: &tui::style::Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let color = Color::from_tui_color(color.clone());
    color.serialize(serializer)
}

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<tui::style::Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let color = Color::deserialize(deserializer)?;
    Ok(color.to_tui_color())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl Color {
    pub fn to_tui_color(&self) -> tui::style::Color {
        match self {
            Color::Reset => tui::style::Color::Reset,
            Color::Black => tui::style::Color::Black,
            Color::Red => tui::style::Color::Red,
            Color::Green => tui::style::Color::Green,
            Color::Yellow => tui::style::Color::Yellow,
            Color::Blue => tui::style::Color::Blue,
            Color::Magenta => tui::style::Color::Magenta,
            Color::Cyan => tui::style::Color::Cyan,
            Color::Gray => tui::style::Color::Gray,
            Color::DarkGray => tui::style::Color::DarkGray,
            Color::LightRed => tui::style::Color::LightRed,
            Color::LightGreen => tui::style::Color::LightGreen,
            Color::LightYellow => tui::style::Color::LightYellow,
            Color::LightBlue => tui::style::Color::LightBlue,
            Color::LightMagenta => tui::style::Color::LightMagenta,
            Color::LightCyan => tui::style::Color::LightCyan,
            Color::White => tui::style::Color::White,
            Color::Rgb(r, g, b) => tui::style::Color::Rgb(*r, *g, *b),
            Color::Indexed(i) => tui::style::Color::Indexed(*i),
        }
    }

    pub fn from_tui_color(color: tui::style::Color) -> Self {
        match color {
            tui::style::Color::Reset => Color::Reset,
            tui::style::Color::Black => Color::Black,
            tui::style::Color::Red => Color::Red,
            tui::style::Color::Green => Color::Green,
            tui::style::Color::Yellow => Color::Yellow,
            tui::style::Color::Blue => Color::Blue,
            tui::style::Color::Magenta => Color::Magenta,
            tui::style::Color::Cyan => Color::Cyan,
            tui::style::Color::Gray => Color::Gray,
            tui::style::Color::DarkGray => Color::DarkGray,
            tui::style::Color::LightRed => Color::LightRed,
            tui::style::Color::LightGreen => Color::LightGreen,
            tui::style::Color::LightYellow => Color::LightYellow,
            tui::style::Color::LightBlue => Color::LightBlue,
            tui::style::Color::LightMagenta => Color::LightMagenta,
            tui::style::Color::LightCyan => Color::LightCyan,
            tui::style::Color::White => Color::White,
            tui::style::Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
            tui::style::Color::Indexed(i) => Color::Indexed(i),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Colors {
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub primary_color: tui::style::Color,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub secondary_color: tui::style::Color,
    #[serde(serialize_with = "serialize_color", deserialize_with = "deserialize_color")]
    pub accent_color: tui::style::Color,
}


impl Colors {
    fn default() -> Self {
        Colors {
            primary_color: tui::style::Color::LightGreen,
            secondary_color: tui::style::Color::LightYellow,
            accent_color: tui::style::Color::LightBlue,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub db_file: String,
    pub date_formats: DateFormats,
    pub show_complete: bool,
    pub icons: Icons,
    pub colors: Colors,
}

impl Settings {
    pub fn set_show_complete(&mut self, show_complete: bool) {
        self.show_complete = show_complete;
        self.save_state();
    }

    pub fn save_state(&self) {
        let settings_path = SettingsBuilder::get_settings_path();
        utils::save_settings(&settings_path, self);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SettingsBuilder {
    pub db_file: String,
    pub date_formats: DateFormats,
    pub show_complete: bool,
    pub icons: Icons,
    pub colors: Colors,
}

impl SettingsBuilder {
    pub fn default() -> Self {
        SettingsBuilder {
            db_file: Self::get_default_db_file(),
            show_complete: true,
            icons: Icons::default(),
            date_formats: DateFormats::new(),
            colors: Colors::default(),
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
            let settings = Self::default();
            let settings_json = serde_json::to_string_pretty(&settings).unwrap();
            fs::write(&path, settings_json).unwrap();
        }
        path
    }

    pub fn build(&mut self) -> Settings {
        Settings {
            db_file: self.db_file.clone(),
            date_formats: self.date_formats.clone(),
            show_complete: self.show_complete,
            icons: self.icons.clone(),
            colors: self.colors.clone(),
        }
    }
}

pub fn get_configuration() -> Settings {
    let settings_path = SettingsBuilder::get_settings_path();
    config::Config::builder()
        .add_source(config::File::with_name(settings_path.as_str()))
        .build()
        .unwrap()
        .try_deserialize::<SettingsBuilder>()
        .expect("Could not deserialize configuration")
        .build()
}
