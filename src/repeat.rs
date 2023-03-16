use crate::day_of_week::DayOfWeek;
use anyhow::Result;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Repeat {
    Never,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    DaysOfWeek(Vec<DayOfWeek>),
}

impl Repeat {
    pub fn parse_from_str(s: &str) -> Result<Repeat> {
        match s.to_lowercase().as_str() {
            "never" | "" => Ok(Repeat::Never),
            "daily" => Ok(Repeat::Daily),
            "weekly" => Ok(Repeat::Weekly),
            "monthly" => Ok(Repeat::Monthly),
            "yearly" => Ok(Repeat::Yearly),
            _ => {
                let days: Vec<Result<DayOfWeek>> = s
                    .split(',')
                    .map(|s| s.trim())
                    .map(DayOfWeek::from_str)
                    .collect();
                if days.iter().any(|d| d.is_err()) {
                    return Err(anyhow::anyhow!("Invalid day of the week"));
                }

                let days: Vec<DayOfWeek> =
                    days.iter().map(|d| d.as_ref().unwrap()).cloned().collect();

                Ok(Repeat::DaysOfWeek(days))
            }
        }
    }
}

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Repeat::Never => write!(f, "Never"),
            Repeat::Daily => write!(f, "Daily"),
            Repeat::Weekly => write!(f, "Weekly"),
            Repeat::Monthly => write!(f, "Monthly"),
            Repeat::Yearly => write!(f, "Yearly"),
            Repeat::DaysOfWeek(days) => {
                let days = days.iter().map(|d| d.to_string()).join(",");
                write!(f, "{}", days)
            }
        }
    }
}
