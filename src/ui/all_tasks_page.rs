use crate::app::App;
use crate::repeat::Repeat;
use crate::task::Task;
use crate::ui::Page;
use crate::utils;
use chrono::{DateTime, Local, TimeZone};
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use tui::layout::{Direction, Rect};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Cell, Row, Table};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    Frame,
};

pub struct AllTasksPage {
    pub show_hidden: bool,
    pub current_idx: Option<usize>,
    pub app: Rc<RefCell<App>>,
}

impl AllTasksPage {
    pub fn new(app: Rc<RefCell<App>>) -> AllTasksPage {
        let show_hidden = app.borrow().settings.show_complete;
        AllTasksPage {
            show_hidden,
            current_idx: None,
            app,
        }
    }

    pub fn get_current_task_id(&self) -> Option<usize> {
        if self.current_idx.is_none() {
            return None;
        }

        let idx = self.current_idx.unwrap();
        Some(self.app.borrow().tasks[idx].id.unwrap())
    }

    pub fn toggle_selected(&mut self) {
        if self.current_idx.is_none() {
            return;
        }

        let task_id = self.get_current_task_id().unwrap();
        self.app.borrow_mut().toggle_complete_task(task_id);

        if !self.show_hidden {
            self.move_closest();
        }
    }

    pub fn delete_selected(&mut self) {
        if self.current_idx.is_none() {
            return;
        }

        let task_id = self.get_current_task_id().unwrap();
        self.app.borrow_mut().delete_task(task_id);
        self.move_closest();
    }

    pub fn next(&mut self) {
        let len = self.app.borrow().tasks.len();

        if self.current_idx.is_none() && len > 0 {
            self.current_idx = Some(0);
            return;
        } else if self.current_idx.is_none() {
            return;
        }

        let curr_idx = self.current_idx.unwrap();

        if self.show_hidden && curr_idx + 1 < len {
            self.current_idx = Some(curr_idx + 1);
            return;
        } else if self.show_hidden {
            return;
        }

        for i in curr_idx + 1..len {
            if !self.app.borrow().tasks[i].complete {
                self.current_idx = Some(i);
                return;
            }
        }
    }

    pub fn prev(&mut self) {
        let len = self.app.borrow().tasks.len();

        if self.current_idx.is_none() && len > 0 {
            self.current_idx = Some(len - 1);
            return;
        } else if self.current_idx.is_none() {
            return;
        }

        let curr_idx = self.current_idx.unwrap();

        if self.show_hidden && curr_idx > 0 {
            self.current_idx = Some((curr_idx + len - 1) % len);
            return;
        }

        for i in (0..curr_idx).rev() {
            if !self.app.borrow().tasks[i].complete {
                self.current_idx = Some(i);
                return;
            }
        }
    }

    pub fn groups(&self) -> Vec<Vec<Task>> {
        self.app
            .borrow()
            .tasks
            .clone()
            .into_iter()
            .group_by(|t| t.date.date_naive())
            .into_iter()
            .map(|(_, group)| group.collect())
            .collect()
    }

    pub fn move_closest(&mut self) {
        let len = self.app.borrow().tasks.len();

        if self.current_idx.is_none() && len > 0 {
            self.current_idx = Some(0);
            return;
        } else if self.current_idx.is_none() {
            return;
        }

        let curr_idx = self.current_idx.unwrap();
        let app = self.app.borrow();

        for i in curr_idx..len {
            if !app.tasks[i].complete {
                self.current_idx = Some(i);
                return;
            }
        }

        for i in (0..curr_idx).rev() {
            if !app.tasks[i].complete {
                self.current_idx = Some(i);
                return;
            }
        }

        self.current_idx = None;
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.app
            .borrow_mut()
            .settings
            .set_show_complete(self.show_hidden);
        if !self.show_hidden {
            self.move_closest();
        }
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

    pub fn open_selected_link(&self) {
        if self.current_idx.is_none() {
            return;
        }

        let task_id = self.get_current_task_id().unwrap();
        let desc_text = self
            .app
            .borrow()
            .get_task(task_id)
            .unwrap()
            .description
            .clone()
            .unwrap_or_default();
        if utils::is_hyperlink(&desc_text) {
            open::that(desc_text).unwrap();
        }
    }
}

impl<B> Page<B> for AllTasksPage
where
    B: Backend,
{
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(area);

        // Build list
        let mut rows = vec![];
        let mut current_idx = 0;
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
                    .fg(Color::LightBlue),
            ));
            rows.push(Row::new(vec![cell]));
            let pre_count = rows.len();

            // All tasks in group
            for (idx, item) in group.iter().enumerate() {
                // Skip if hidden
                if !self.show_hidden && item.complete {
                    current_idx += 1;
                    continue;
                }

                // Create string
                let complete_icon = self.get_complete_icon(item.complete);
                let recurring_icon = self.get_repeats_icon(&item.repeats);
                let title = format!("{} {} {} ", complete_icon, item.name, recurring_icon);
                let title_style = match (item.complete, self.current_idx) {
                    (_, Some(idx)) if idx == current_idx => Style::default()
                        .fg(Color::LightYellow)
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

                current_idx += 1;
                rows.push(new_row);
            }

            // If no tasks in group, pop the group title
            if rows.len() == pre_count {
                rows.pop();
            }
        }
        let border_style = match focused {
            true => Style::default().fg(Color::LightYellow),
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
