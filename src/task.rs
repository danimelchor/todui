use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{repeat::Repeat, utils};

pub fn serialize_dt<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let s = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap();
    Ok(dt)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub date: NaiveDate,
    pub repeats: Repeat,
    pub description: Option<String>,
    pub completed: bool,
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = utils::date_to_str(&self.date);
        let repeats = self.repeats.to_string();
        let completed = if self.completed { "[x]" } else { "[ ]" };
        write!(f, "{} {}\t\t{}\t\t{}", completed, self.name, date, repeats)
    }
}

impl Task {
    pub fn new() -> Task {
        Task {
            name: "".to_string(),
            date: Local::now().naive_local().date(),
            repeats: Repeat::Never,
            description: None,
            completed: false,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_date(&mut self, date: NaiveDate) {
        self.date = date;
    }

    pub fn set_repeats(&mut self, repeats: Repeat) {
        self.repeats = repeats;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_completed(&mut self) -> Option<Task> {
        self.completed = true;
        let date = match &self.repeats {
            Repeat::DaysOfWeek(days) => {
                let today = self.date.weekday();
                let next_day = days
                    .iter()
                    .map(|d| d.to_int())
                    .sorted()
                    .find(|d| *d > today.num_days_from_monday())
                    .unwrap_or(7);
                Some(self.date + chrono::Duration::days(next_day as i64))
            }
            Repeat::Never => None,
            Repeat::Daily => Some(self.date + chrono::Duration::days(1)),
            Repeat::Weekly => Some(self.date + chrono::Duration::weeks(1)),
            Repeat::Monthly => Some(self.date + chrono::Duration::weeks(4)),
            Repeat::Yearly => Some(self.date + chrono::Duration::weeks(52)),
        };

        if let Some(date) = date {
            let mut new_task = self.clone();
            new_task.set_date(date);
            new_task.set_incomplete();
            Some(new_task)
        } else {
            None
        }
    }

    pub fn set_incomplete(&mut self) -> Option<Task> {
        self.completed = false;
        None
    }

    pub fn toggle_completed(&mut self) -> Option<Task> {
        if self.completed {
            self.set_incomplete()
        } else {
            self.set_completed()
        }
    }
}
