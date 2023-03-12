use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    pub fn to_int(&self) -> u32 {
        match self {
            DayOfWeek::Monday => 1,
            DayOfWeek::Tuesday => 2,
            DayOfWeek::Wednesday => 3,
            DayOfWeek::Thursday => 4,
            DayOfWeek::Friday => 5,
            DayOfWeek::Saturday => 6,
            DayOfWeek::Sunday => 7,
        }
    }

    pub fn from_str(s: &str) -> Result<DayOfWeek> {
        match s {
            "Mon" => Ok(DayOfWeek::Monday),
            "Tue" => Ok(DayOfWeek::Tuesday),
            "Wed" => Ok(DayOfWeek::Wednesday),
            "Thu" => Ok(DayOfWeek::Thursday),
            "Fri" => Ok(DayOfWeek::Friday),
            "Sat" => Ok(DayOfWeek::Saturday),
            "Sun" => Ok(DayOfWeek::Sunday),
            _ => Err(anyhow::anyhow!("Invalid day of the week")),
        }
    }

    pub fn from_chrono(day: chrono::Weekday) -> Self {
        match day {
            chrono::Weekday::Mon => Self::Monday,
            chrono::Weekday::Tue => Self::Tuesday,
            chrono::Weekday::Wed => Self::Wednesday,
            chrono::Weekday::Thu => Self::Thursday,
            chrono::Weekday::Fri => Self::Friday,
            chrono::Weekday::Sat => Self::Saturday,
            chrono::Weekday::Sun => Self::Sunday,
        }
    }
}

impl Display for DayOfWeek {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DayOfWeek::Monday => write!(f, "Mon"),
            DayOfWeek::Tuesday => write!(f, "Tue"),
            DayOfWeek::Wednesday => write!(f, "Wed"),
            DayOfWeek::Thursday => write!(f, "Thu"),
            DayOfWeek::Friday => write!(f, "Fri"),
            DayOfWeek::Saturday => write!(f, "Sat"),
            DayOfWeek::Sunday => write!(f, "Sun"),
        }
    }
}
