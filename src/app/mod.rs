use crate::{configuration::Settings, task::Task, utils};

pub mod all_tasks;
pub mod new_task;

use self::all_tasks::AllTasksPage;
use self::new_task::NewTaskPage;

pub enum AppPage {
    AllTasks,
    NewTask,
}

pub struct App<'a> {
    pub tasks: Vec<Task>,
    pub settings: Settings,
    pub current_page: AppPage,
    pub all_task_page: &'a mut AllTasksPage,
    pub new_task_page: &'a mut NewTaskPage,
}

impl<'a> App<'a> {
    pub fn new(settings: Settings) -> App<'a> {
        let tasks = utils::load_tasks(&settings.store_file);
        App {
            tasks,
            settings,
            current_page: AppPage::AllTasks,
            all_task_page: &mut AllTasksPage::new(&self),
            new_task_page: &mut NewTaskPage::new(&self),
        }
    }

    pub fn save_state(&mut self) {
        utils::save_tasks(&self.settings.store_file, &self);
    }
}
