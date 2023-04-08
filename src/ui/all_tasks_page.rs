use crate::app::App;
use crate::repeat::Repeat;
use crate::task::Task;
use crate::ui::Page;
use crate::utils;
use anyhow::Result;
use chrono::{DateTime, Local, TimeZone};
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use tui::layout::{Direction, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Row, Table, Tabs};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    Frame,
};

pub struct AllTasksPage {
    pub show_hidden: bool,
    pub current_id: Option<usize>,
    pub app: Rc<RefCell<App>>,

    current_group: Option<String>,
}

impl AllTasksPage {
    pub fn new(app: Rc<RefCell<App>>) -> AllTasksPage {
        let show_hidden = app.borrow().settings.show_complete;
        let current_group = app.borrow().settings.current_group.clone();

        let mut atp = AllTasksPage {
            show_hidden,
            current_id: None,
            current_group,
            app,
        };

        let any_in_group = atp
            .visible_tasks()
            .iter()
            .any(|t| t.group == atp.current_group);
        if !any_in_group {
            atp.set_group(None);
        }

        atp
    }

    /// Returns the tasks that should be displayed on the page
    pub fn visible_tasks(&self) -> Vec<Task> {
        let app = self.app.borrow_mut();
        let tasks: Vec<&Task> = app.tasks.values().collect();

        let tasks: Vec<&Task> = if !self.show_hidden {
            tasks.into_iter().filter(|t| !t.complete).collect()
        } else {
            tasks
        };

        // Filter out tasks not in the current group
        let tasks: Vec<&Task> = if let Some(group) = &self.current_group {
            tasks
                .into_iter()
                .filter(|t| t.group.is_some())
                .filter(|t| t.group.as_ref().unwrap() == group)
                .collect()
        } else {
            tasks
        };

        tasks
            .into_iter()
            .cloned()
            .sorted_by(|a, b| a.date.cmp(&b.date))
            .collect()
    }

    pub fn ensure_group_exists(&mut self) {
        // Check that there are still visible tasks in group
        let any = self
            .visible_tasks()
            .iter()
            .any(|t| t.group == self.get_current_group());
        if !any {
            self.set_group(None);
        }
    }

    pub fn ensure_task_exists(&mut self) {
        // Check that the current task still exists
        if let Some(id) = self.current_id {
            let any = self.visible_tasks().iter().any(|t| t.id.unwrap() == id);
            if !any {
                self.current_id = None;
            }
        }
    }

    /// Toggles the complete status of the currently selected task
    pub fn toggle_selected(&mut self) {
        if let Some(task_id) = self.current_id {
            self.app.borrow_mut().toggle_complete_task(task_id);

            if !self.show_hidden {
                self.move_closest();
            }
        }
        self.ensure_group_exists();
    }

    pub fn next(&mut self) {
        let tasks = self.visible_tasks();
        match self.current_id {
            Some(id) => {
                let idx = tasks.iter().position(|t| t.id.unwrap() == id).unwrap();
                if idx < tasks.len() - 1 {
                    self.current_id = Some(tasks[idx + 1].id.unwrap());
                }
            }
            None => {
                if !tasks.is_empty() {
                    self.current_id = Some(tasks[0].id.unwrap());
                }
            }
        }
    }

    pub fn prev(&mut self) {
        let tasks = self.visible_tasks();
        match self.current_id {
            Some(id) => {
                let idx = tasks.iter().position(|t| t.id.unwrap() == id).unwrap();
                if idx > 0 {
                    self.current_id = Some(tasks[idx - 1].id.unwrap());
                }
            }
            None => {
                if !tasks.is_empty() {
                    self.current_id = Some(tasks[tasks.len() - 1].id.unwrap());
                }
            }
        }
    }

    pub fn get_current_group(&self) -> Option<String> {
        self.current_group.clone()
    }

    pub fn set_group(&mut self, group: Option<String>) {
        self.current_group = group.clone();
        if self.current_id.is_some() {
            let id = self.current_id.unwrap();
            let visible_tasks = self.visible_tasks();
            if !visible_tasks.iter().any(|t| t.id.unwrap() == id) {
                self.current_id = None;
            }
        }
        self.app.borrow_mut().settings.set_current_group(group);
    }

    pub fn next_group(&mut self) {
        let groups = self.get_groups();
        self.current_id = None;
        match &self.current_group {
            Some(group) => {
                let idx = groups.iter().position(|g| g == group).unwrap();
                if idx < groups.len() - 1 {
                    self.current_group = Some(groups[idx + 1].clone());
                }
            }
            None => {
                if groups.len() > 1 {
                    self.current_group = Some(groups[1].clone());
                }
            }
        }
        self.app
            .borrow_mut()
            .settings
            .set_current_group(self.current_group.clone());
    }

    pub fn prev_group(&mut self) {
        let groups = self.get_groups();
        self.current_id = None;
        match &self.current_group {
            Some(group) => {
                let idx = groups.iter().position(|g| g == group).unwrap();
                if idx > 1 {
                    self.current_group = Some(groups[idx - 1].clone());
                } else {
                    self.current_group = None;
                }
            }
            None => {}
        }
        self.app
            .borrow_mut()
            .settings
            .set_current_group(self.current_group.clone());
    }

    pub fn groups(&self) -> Vec<Vec<Task>> {
        self.visible_tasks()
            .into_iter()
            .group_by(|t| t.date.date_naive())
            .into_iter()
            .map(|(_, group)| {
                group
                    .sorted_by(|a, b| a.date.cmp(&b.date))
                    .collect::<Vec<Task>>()
            })
            .collect()
    }

    pub fn move_closest(&mut self) {
        let current_date: Option<DateTime<Local>> = {
            match self.current_id {
                Some(id) => self.app.borrow().get_task(id).map(|t| t.date),
                None => None,
            }
        };

        // Move to next task if any, else previous, else none
        let tasks = self.visible_tasks();
        let current_date = current_date.unwrap_or_else(Local::now);
        let closest = tasks.iter().min_by_key(|t| {
            t.date
                .signed_duration_since(current_date)
                .num_seconds()
                .abs()
        });
        match closest {
            Some(task) => self.current_id = Some(task.id.unwrap()),
            None => self.current_id = None,
        }
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.app
            .borrow_mut()
            .settings
            .set_show_complete(self.show_hidden);
        self.ensure_group_exists();
        if !self.show_hidden {
            self.move_closest();
        }
    }

    pub fn get_groups(&self) -> Vec<String> {
        let mut groups = vec!["All Tasks".to_string()];
        let tasks: Vec<Task> = self.app.borrow().tasks.values().cloned().collect();

        let tasks: Vec<Task> = if !self.show_hidden {
            tasks.into_iter().filter(|t| !t.complete).collect()
        } else {
            tasks
        };
        let mut other_groups =  tasks
            .iter()
            .filter_map(|t| t.group.clone())
            .unique()
            .collect::<Vec<String>>();
        groups.append(&mut other_groups);
        groups
    }

    pub fn get_complete_icon(&self, complete: bool) -> String {
        self.app.borrow().settings.icons.get_complete_icon(complete)
    }

    pub fn get_repeats_icon(&self, repeats: &Repeat) -> String {
        match repeats {
            Repeat::Never => String::from(""),
            _ => self.app.borrow().settings.icons.repeats.clone(),
        }
    }

    pub fn date_to_str(&self, date: &DateTime<Local>) -> String {
        utils::date_to_display_str(date, &self.app.borrow().settings)
    }

    pub fn open_selected_link(&self) -> Result<()> {
        if let Some(task_id) = self.current_id {
            let url = self
                .app
                .borrow()
                .get_task(task_id)
                .unwrap()
                .url
                .clone()
                .unwrap_or_default();

            if !url.is_empty() {
                open::that(url)?;
            }
        }
        Ok(())
    }

    pub fn get_primary_color(&self) -> Color {
        self.app.borrow().settings.colors.primary_color
    }

    pub fn get_secondary_color(&self) -> Color {
        self.app.borrow().settings.colors.secondary_color
    }

    pub fn get_accent_color(&self) -> Color {
        self.app.borrow().settings.colors.accent_color
    }
}

impl<B> Page<B> for AllTasksPage
where
    B: Backend,
{
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        // Render tabs
        let groups = self.get_groups();
        let titles = groups
            .iter()
            .map(|t| Spans::from(Span::styled(t, Style::default().fg(Color::White))))
            .collect();
        let current_group_idx = match &self.current_group {
            None => 0,
            Some(group) => groups.iter().position(|g| g == group).unwrap(),
        };
        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Groups"))
            .select(current_group_idx)
            // .style(Style::default().fg(self.get_primary_color()))
            .highlight_style(
                Style::default()
                    .fg(self.get_secondary_color())
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(tabs, chunks[0]);

        // Build list
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[1]);

        let mut rows = vec![];
        for group in self.groups() {
            // Group title
            let group_date = &group[0].date.date_naive().and_hms_opt(23, 59, 59).unwrap();
            let group_date = Local.from_local_datetime(group_date).unwrap();
            let date_str = self.date_to_str(&group_date).to_uppercase();
            let group_title = " ".to_string() + date_str.as_str();
            let cell = Cell::from(Span::styled(
                group_title,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(self.get_accent_color()),
            ));
            rows.push(Row::new(vec![cell]));
            let pre_count = rows.len();

            // All tasks in group
            for (idx, item) in group.iter().enumerate() {
                // Skip if hidden
                if !self.show_hidden && item.complete {
                    continue;
                }

                // Create string
                let complete_icon = self.get_complete_icon(item.complete);
                let recurring_icon = self.get_repeats_icon(&item.repeats);
                let title = format!("{} {} {} ", complete_icon, item.name, recurring_icon);
                let title_style = match (item.complete, self.current_id) {
                    (_, Some(task_id)) if task_id == item.id.unwrap() => Style::default()
                        .fg(self.get_secondary_color())
                        .add_modifier(Modifier::BOLD),
                    (true, _) => Style::default().fg(Color::DarkGray),
                    _ => Style::default().fg(Color::White),
                };
                let title_style = title_style.add_modifier(Modifier::BOLD);
                let title_cell = Spans::from(Span::styled(title, title_style));

                // Create row
                let cell = Cell::from(title_cell);
                let mut new_row = Row::new(vec![cell]);

                // Add bottom margin if last item in group
                if idx == group.len() - 1 {
                    new_row = new_row.bottom_margin(1);
                }

                rows.push(new_row);
            }

            // If no tasks in group, pop the group title
            if rows.len() == pre_count {
                rows.pop();
            }
        }
        let border_style = match focused {
            true => Style::default().fg(self.get_primary_color()),
            false => Style::default(),
        };
        let border_type = match focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };
        let list = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Todos")
                    .border_style(border_style)
                    .border_type(border_type),
            )
            .widths(&[Constraint::Percentage(100)]);
        f.render_widget(list, chunks[0]);
    }
}
