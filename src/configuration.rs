use crate::utils;
use anyhow::{anyhow, Result};
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Icons {
    pub complete: String,
    pub incomplete: String,
    pub repeats: String,
}

impl Icons {
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

impl Default for Icons {
    fn default() -> Self {
        Icons {
            complete: "󰄴".to_string(),
            incomplete: "󰝦".to_string(),
            repeats: "".to_string(),
        }
    }
}

pub fn serialize_color<S>(color: &tui::style::Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let color = Color::from_tui_color(*color);
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

pub fn deserialize_key<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_ref() {
        "esc" => Ok(KeyCode::Esc),
        "backspace" => Ok(KeyCode::Backspace),
        "left" => Ok(KeyCode::Left),
        "right" => Ok(KeyCode::Right),
        "up" => Ok(KeyCode::Up),
        "down" => Ok(KeyCode::Down),
        "home" => Ok(KeyCode::Home),
        "end" => Ok(KeyCode::End),
        "delete" => Ok(KeyCode::Delete),
        "insert" => Ok(KeyCode::Insert),
        "pageup" => Ok(KeyCode::PageUp),
        "pagedown" => Ok(KeyCode::PageDown),
        "f1" => Ok(KeyCode::F(1)),
        "f2" => Ok(KeyCode::F(2)),
        "f3" => Ok(KeyCode::F(3)),
        "f4" => Ok(KeyCode::F(4)),
        "f5" => Ok(KeyCode::F(5)),
        "f6" => Ok(KeyCode::F(6)),
        "f7" => Ok(KeyCode::F(7)),
        "f8" => Ok(KeyCode::F(8)),
        "f9" => Ok(KeyCode::F(9)),
        "f10" => Ok(KeyCode::F(10)),
        "f11" => Ok(KeyCode::F(11)),
        "f12" => Ok(KeyCode::F(12)),
        "space" => Ok(KeyCode::Char(' ')),
        "tab" => Ok(KeyCode::Tab),
        "enter" => Ok(KeyCode::Enter),
        c if c.len() == 1 => Ok(KeyCode::Char(c.chars().next().unwrap())),
        _ => Err(serde::de::Error::custom("Invalid key")),
    }
}

pub fn serialize_key<S>(key: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let key = KeyBindings::key_to_str(key);
    key.serialize(serializer)
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeyBindings {
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub quit: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub down: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub up: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub complete_task: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub toggle_completed_tasks: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub delete_task: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub new_task: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub edit_task: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub save_changes: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub enter_insert_mode: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub enter_normal_mode: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub go_back: KeyCode,
    #[serde(deserialize_with = "deserialize_key", serialize_with = "serialize_key")]
    pub open_link: KeyCode,
}

impl KeyBindings {
    pub fn key_to_str(key: &KeyCode) -> String {
        match key {
            KeyCode::Esc => "Esc".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::F(1) => "F1".to_string(),
            KeyCode::F(2) => "F2".to_string(),
            KeyCode::F(3) => "F3".to_string(),
            KeyCode::F(4) => "F4".to_string(),
            KeyCode::F(5) => "F5".to_string(),
            KeyCode::F(6) => "F6".to_string(),
            KeyCode::F(7) => "F7".to_string(),
            KeyCode::F(8) => "F8".to_string(),
            KeyCode::F(9) => "F9".to_string(),
            KeyCode::F(10) => "F10".to_string(),
            KeyCode::F(11) => "F11".to_string(),
            KeyCode::F(12) => "F12".to_string(),
            KeyCode::Char(' ') => "Space".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Char(c) => c.to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: KeyCode::Char('q'),
            down: KeyCode::Char('j'),
            up: KeyCode::Char('k'),
            complete_task: KeyCode::Char('x'),
            toggle_completed_tasks: KeyCode::Char('h'),
            delete_task: KeyCode::Char('d'),
            new_task: KeyCode::Char('n'),
            edit_task: KeyCode::Char('e'),
            save_changes: KeyCode::Enter,
            enter_insert_mode: KeyCode::Char('i'),
            enter_normal_mode: KeyCode::Esc,
            go_back: KeyCode::Char('b'),
            open_link: KeyCode::Enter,
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
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub primary_color: tui::style::Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
    pub secondary_color: tui::style::Color,
    #[serde(
        serialize_with = "serialize_color",
        deserialize_with = "deserialize_color"
    )]
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
    pub date_formats: DateFormats,
    pub show_complete: bool,
    pub icons: Icons,
    pub colors: Colors,
    pub keybindings: KeyBindings,
}

impl Settings {
    pub fn set_show_complete(&mut self, show_complete: bool) {
        self.show_complete = show_complete;
        self.save_state();
    }

    pub fn save_state(&self) {
        let settings_path =
            SettingsBuilder::get_settings_path().expect("Settings file should exist.");
        utils::save_settings(&settings_path, self);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SettingsBuilder {
    pub date_formats: DateFormats,
    pub show_complete: bool,
    pub icons: Icons,
    pub colors: Colors,
    pub keybindings: KeyBindings,
}

impl SettingsBuilder {
    pub fn default_path() -> Result<PathBuf> {
        match dirs::home_dir() {
            Some(path) => {
                let path = Path::new(&path);

                let config_dir = path.join(".config");
                if !path.exists() {
                    fs::create_dir_all(path)?;
                }

                let todo_dir = config_dir.join("todo-rs");
                if !todo_dir.exists() {
                    fs::create_dir_all(&todo_dir)?;
                }
                Ok(todo_dir)
            }
            None => Err(anyhow!("Could not find home directory")),
        }
    }

    pub fn get_default_db_file() -> Result<PathBuf> {
        let default_path = Self::default_path()?;
        let path = default_path.join("tasks.json");

        if !path.exists() {
            let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
            writeln!(file, "[]")?;
        }

        Ok(path)
    }

    pub fn get_settings_path() -> Result<PathBuf> {
        let default_path = Self::default_path()?;
        let path = default_path.join("settings.json");
        if !path.exists() {
            let settings = Self::default();
            let settings_json = serde_json::to_string_pretty(&settings)?;
            fs::write(&path, settings_json)?;
        }
        Ok(path)
    }

    pub fn build(&mut self) -> Settings {
        Settings {
            date_formats: self.date_formats.clone(),
            show_complete: self.show_complete,
            icons: self.icons.clone(),
            colors: self.colors.clone(),
            keybindings: self.keybindings.clone(),
        }
    }
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        SettingsBuilder {
            show_complete: true,
            icons: Icons::default(),
            date_formats: DateFormats::new(),
            colors: Colors::default(),
            keybindings: KeyBindings::default(),
        }
    }
}

pub fn get_configuration() -> Settings {
    let settings_path = SettingsBuilder::get_settings_path().expect("Settings file should exist.");
    let file = OpenOptions::new()
        .read(true)
        .open(settings_path)
        .expect("Could not open settings file");
    serde_json::from_reader(file).expect("Could not parse settings file")
}

pub fn get_db_file() -> PathBuf {
    SettingsBuilder::get_default_db_file().expect("Could not find default db file")
}
