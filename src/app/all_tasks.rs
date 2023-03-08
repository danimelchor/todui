use crate::app::App;
use crate::task::Task;
use itertools::Itertools;

pub struct AllTasksPage<'a> {
    pub show_hidden: bool,
    pub current_idx: usize,
    pub app: &'a mut App,
}

impl<'a> AllTasksPage<'a> {
    pub fn new(app: &mut App) -> AllTasksPage {
        AllTasksPage {
            show_hidden: true,
            current_idx: 0,
            app,
        }
    }

    pub fn toggle_selected(&mut self) {
        let new_task = self.app.tasks[self.current_idx].toggle_completed();
        if let Some(new_task) = new_task {
            self.app.tasks.push(new_task);
        }
        self.app.tasks.sort_by(|a, b| a.date.cmp(&b.date));

        if !self.show_hidden {
            self.move_closest();
        }
    }

    pub fn delete_selected(&mut self) {
        self.app.tasks.remove(self.current_idx);
        self.move_closest();
    }

    pub fn next(&mut self) {
        let mut next_idx: Option<usize> = None;
        if self.show_hidden {
            if self.current_idx < self.app.tasks.len() - 1 {
                next_idx = Some(self.current_idx + 1);
            }
        } else {
            // Find next non-completed task if any
            for (i, task) in self.app.tasks.iter().enumerate() {
                if i > self.current_idx && !task.completed {
                    next_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = next_idx {
            self.current_idx = idx;
        }
    }

    pub fn prev(&mut self) {
        let mut prev_idx: Option<usize> = None;
        if self.show_hidden {
            if self.current_idx > 0 {
                prev_idx = Some(self.current_idx - 1);
            }
        } else {
            // Find previous non-completed task if any
            for (i, task) in self.app.tasks.iter().enumerate().rev() {
                if i < self.current_idx && !task.completed {
                    prev_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = prev_idx {
            self.current_idx = idx;
        }
    }

    pub fn groups(&self) -> Vec<Vec<Task>> {
        let mut tasks = self.app.tasks.clone();
        tasks.sort_by(|a, b| a.date.cmp(&b.date));
        tasks
            .into_iter()
            .group_by(|t| t.date)
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect()
    }

    pub fn move_closest(&mut self) {
        let mut prev_dist = 0;
        let mut prev_idx = None;

        let mut next_dist = 0;
        let mut next_idx = None;

        // Find next possible
        for (i, task) in self.app.tasks.iter().enumerate() {
            if i > self.current_idx && !task.completed {
                next_idx = Some(i);
                next_dist = i - self.current_idx;
                break;
            }
        }

        // Find previous possible
        for (i, task) in self.app.tasks.iter().enumerate().rev() {
            if i < self.current_idx && !task.completed {
                prev_idx = Some(i);
                prev_dist = self.current_idx - i;
                break;
            }
        }

        if let Some(idx) = prev_idx {
            if let Some(next_idx) = next_idx {
                if prev_dist < next_dist {
                    self.current_idx = idx;
                } else {
                    self.current_idx = next_idx;
                }
            } else {
                self.current_idx = idx;
            }
        } else if let Some(next_idx) = next_idx {
            self.current_idx = next_idx;
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;

        // Find closest non-completed task if hidden
        if !self.show_hidden {
            self.move_closest();
        }
    }
}
