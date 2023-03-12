use crate::{configuration::Settings, task::Task, utils};

pub struct App {
    pub tasks: Vec<Task>,
    pub settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let tasks = utils::load_tasks(&settings.db_file);
        App { tasks, settings }
    }

    pub fn save_state(&mut self) {
        self.tasks.sort_by(|a, b| a.date.cmp(&b.date));
        utils::save_tasks(&self.settings.db_file, &self);
    }

    pub fn get_task(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }

    pub fn add_task(&mut self, t: Task) {
        self.tasks.push(t);
        self.save_state();
    }

    pub fn delete_task(&mut self, index: usize) {
        self.tasks.remove(index);
        self.save_state();
    }

    pub fn toggle_completed_task(&mut self, index: usize) {
        let new_task = self.tasks[index].toggle_completed();
        if let Some(new_task) = new_task {
            self.add_task(new_task);
            self.delete_task(index);
        }

        self.save_state();
    }
}
