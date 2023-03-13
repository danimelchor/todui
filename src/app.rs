use crate::{configuration::Settings, task::Task, utils};

pub struct App {
    pub tasks: Vec<Task>,
    pub settings: Settings,
    pub current_id: usize,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let tasks = utils::load_tasks(&settings.db_file);
        let current_id = tasks.iter().map(|t| t.id.unwrap()).max().unwrap_or(0);
        App {
            tasks,
            settings,
            current_id,
        }
    }

    pub fn save_state(&mut self) {
        self.tasks.sort_by(|a, b| a.date.cmp(&b.date));
        utils::save_tasks(&self.settings.db_file, &self);
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id.unwrap() == id)
    }

    pub fn add_task(&mut self, mut t: Task) -> Task {
        t.id = Some(self.get_next_id());
        self.tasks.push(t.clone());
        self.save_state();
        t
    }

    pub fn delete_task(&mut self, id: usize) -> Option<Task> {
        let deleted_idx = self.tasks.iter().position(|t| t.id.unwrap() == id)?;
        let deleted_task = self.tasks.get(deleted_idx).cloned();
        self.tasks.remove(deleted_idx);
        self.save_state();
        deleted_task
    }

    pub fn set_complete(&mut self, id: usize, complete: bool) -> Option<Task> {
        let idx = self.tasks.iter().position(|t| t.id.unwrap() == id)?;
        let new_task;
        if complete {
            let possible_new_task = self.tasks[idx].set_complete();
            if let Some(possible_new_task) = possible_new_task {
                new_task = self.add_task(possible_new_task);
                self.delete_task(id);
            } else {
                new_task = self.tasks[idx].clone();
            }
        } else {
            self.tasks[idx].set_incomplete();
            new_task = self.tasks[idx].clone();
        }

        self.save_state();
        Some(new_task)
    }

    pub fn toggle_complete_task(&mut self, id: usize) -> Option<Task> {
        let idx = self.tasks.iter().position(|t| t.id.unwrap() == id).unwrap();
        let complete = self.tasks[idx].complete;
        self.set_complete(id, !complete)
    }

    fn get_next_id(&mut self) -> usize {
        self.current_id += 1;
        self.current_id
    }
}
