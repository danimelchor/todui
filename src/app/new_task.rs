use crate::{repeat::Repeat, task::Task};
use anyhow::{Context, Result};
use chrono::NaiveDate;

use super::{App, AppPage};

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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_date(&mut self, date: String) {
        self.date = date;
    }

    pub fn set_repeats(&mut self, repeats: String) {
        self.repeats = repeats;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
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

#[derive(PartialEq)]
pub enum NewTaskInputMode {
    Normal,
    Editing,
}

pub struct NewTaskPage<'a> {
    pub task_form: TaskForm,
    pub input_mode: NewTaskInputMode,
    pub current_idx: usize,
    pub num_fields: usize,
    pub error: Option<String>,
    pub app: &'a mut App,
}

impl<'a> NewTaskPage<'a> {
    pub fn new(app: &mut App) -> NewTaskPage {
        NewTaskPage {
            task_form: TaskForm::new(),
            input_mode: NewTaskInputMode::Normal,
            current_idx: 0,
            error: None,
            num_fields: 4,
            app,
        }
    }

    pub fn next_field(&mut self) {
        if self.current_idx < self.num_fields - 1 {
            self.current_idx += 1;
        }
    }

    pub fn prev_field(&mut self) {
        if self.current_idx > 0 {
            self.current_idx -= 1;
        }
    }

    pub fn submit(&mut self) {
        let result = self.task_form.submit();
        if let Ok(new_task) = result {
            self.app.tasks.push(new_task);
            self.input_mode = NewTaskInputMode::Normal;
            self.task_form = TaskForm::new();
            self.app.current_page = AppPage::AllTasks;
        } else {
            self.error = Some(result.err().unwrap().to_string());
        }
    }

    pub fn add_char(&mut self, c: char) {
        match self.current_idx {
            0 => {
                self.task_form.name.push(c);
            }
            1 => {
                self.task_form.date.push(c);
            }
            2 => {
                self.task_form.repeats.push(c);
            }
            3 => {
                self.task_form.description.push(c);
            }
            _ => {}
        };
    }

    pub fn remove_char(&mut self) {
        match self.current_idx {
            0 => {
                self.task_form.name.pop();
            }
            1 => {
                self.task_form.date.pop();
            }
            2 => {
                self.task_form.repeats.pop();
            }
            3 => {
                self.task_form.description.pop();
            }
            _ => {}
        };
    }
}
