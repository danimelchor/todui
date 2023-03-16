use anyhow::Context;
use anyhow::Result;

use crate::configuration::Settings;
use crate::repeat::Repeat;
use crate::task::Task;
use crate::utils;

#[derive(Default)]
pub struct TaskForm {
    pub id: Option<usize>,
    pub name: String,
    pub date: String,
    pub repeats: String,
    pub group: String,
    pub description: String,
    pub url: String,
}

impl TaskForm {
    pub fn from_task(task: &Task, settings: &Settings) -> Self {
        Self {
            id: task.id,
            name: task.name.to_string(),
            date: utils::date_to_input_str(&task.date, settings),
            repeats: task.repeats.to_string(),
            group: task.group.clone().unwrap_or_default(),
            description: task.description.clone().unwrap_or_default(),
            url: task.url.clone().unwrap_or_default(),
        }
    }

    pub fn submit(&mut self, settings: &Settings) -> Result<Task> {
        let mut task = Task::default();

        let repeat = Repeat::parse_from_str(&self.repeats).context("Invalid repeat format")?;
        let date = utils::parse_date(&self.date, settings).unwrap_or(utils::get_today());

        if self.name.is_empty() {
            return Err(anyhow::anyhow!("Task name cannot be empty"));
        }

        task.set_id(self.id);
        task.set_name(self.name.clone());
        task.set_date(date);
        task.set_repeats(repeat);
        if !self.group.is_empty() {
            task.set_group(self.group.clone());
        }
        if !self.description.is_empty() {
            task.set_description(self.description.clone());
        }
        if !self.url.is_empty() {
            task.set_url(self.url.clone());
        }

        Ok(task)
    }
}
