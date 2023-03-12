use anyhow::Context;
use anyhow::Result;
use chrono::NaiveDate;

use crate::repeat::Repeat;
use crate::task::Task;

pub struct TaskForm {
    pub name: String,
    pub date: String,
    pub repeats: String,
    pub description: String,
}

impl TaskForm {
    pub fn new() -> TaskForm {
        TaskForm {
            name: "".to_string(),
            date: "".to_string(),
            repeats: "".to_string(),
            description: "".to_string(),
        }
    }

    pub fn submit(&mut self) -> Result<Task> {
        let mut task = Task::new();

        let repeat = Repeat::parse_from_str(&self.repeats).context("Invalid repeat format")?;
        let date =
            NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").context("Invalid date format")?;

        task.set_name(self.name.clone());
        task.set_date(date);
        task.set_repeats(repeat);
        task.set_description(self.description.clone());

        Ok(task)
    }
}
