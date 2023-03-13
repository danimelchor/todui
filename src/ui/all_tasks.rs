use crate::app::App;
use crate::task::Task;
use crate::ui::{Page, UIPage};
use crate::utils;
use anyhow::Result;
use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use tui::text::Span;
use tui::widgets::BorderType;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
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
            .group_by(|t| t.date)
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

    pub fn get_icon(&self, complete: bool) -> String {
        self.app.borrow().settings.icons.get_icon(complete)
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

struct Widths {
    name: usize,
    date: usize,
    repeats_every: usize,
}

impl<B> Page<B> for AllTasksPage
where
    B: Backend,
{
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<UIPage> {
        terminal.draw(|f| self.ui(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(UIPage::Quit),
                KeyCode::Char('j') => self.next(),
                KeyCode::Char('k') => self.prev(),
                KeyCode::Char('x') => self.toggle_selected(),
                KeyCode::Char('h') => self.toggle_hidden(),
                KeyCode::Char('d') => self.delete_selected(),
                KeyCode::Enter => self.open_selected_link(),
                KeyCode::Char('n') => return Ok(UIPage::NewTask),
                KeyCode::Char('e') => {
                    let task_id = self.get_current_task_id().unwrap();
                    return Ok(UIPage::EditTask(task_id));
                }
                _ => {}
            }
        }

        Ok(UIPage::SamePage)
    }

    fn ui(&self, f: &mut Frame<B>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(1)
            .split(f.size());

        let header_cells = ["Done", "Name", "Date", "Repeats every", "Description"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        // Rows
        let selected_style = Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD);
        let complete_style = Style::default().fg(Color::DarkGray);
        let default_style = Style::default().fg(Color::White);

        let mut rows: Vec<Row> = vec![];
        let mut current_idx = 0;
        let mut widths = Widths {
            name: 0,
            date: 4,
            repeats_every: 13,
        };
        for group in self.groups() {
            for (item_idx, item) in group.iter().enumerate() {
                let date_str = self.date_to_str(&item.date);
                let repeats_str = item.repeats.to_string();

                widths.date = max(widths.date, date_str.len());
                widths.repeats_every = max(widths.repeats_every, repeats_str.len());

                // Name cell
                widths.name = max(widths.name, item.name.len());
                let name_cell = if !item.complete {
                    Cell::from(item.name.clone())
                } else {
                    Cell::from(Span::styled(
                        item.name.clone(),
                        Style::default().add_modifier(Modifier::CROSSED_OUT),
                    ))
                };

                // Description cell
                let desc_text = item.description.clone().unwrap_or_default();
                let desc_cell = if utils::is_hyperlink(&desc_text) {
                    let color = match (self.current_idx, item.complete) {
                        (Some(idx), _) if idx == current_idx => Color::LightBlue,
                        (_, true) => Color::DarkGray,
                        _ => Color::White,
                    };
                    Cell::from(Span::styled(
                        "Open link",
                        Style::default()
                            .fg(color)
                            .add_modifier(Modifier::UNDERLINED),
                    ))
                } else {
                    Cell::from(desc_text)
                };

                let cells = vec![
                    Cell::from(self.get_icon(item.complete)),
                    name_cell,
                    Cell::from(date_str),
                    Cell::from(repeats_str),
                    desc_cell,
                ];

                let style = match item.complete {
                    true => complete_style,
                    false => default_style,
                };

                let style = match self.current_idx {
                    Some(idx) if idx == current_idx => selected_style,
                    _ => style,
                };
                current_idx += 1;

                let mut new_row = Row::new(cells).style(style);

                if item_idx == group.len() - 1 {
                    new_row = new_row.bottom_margin(1);
                }

                if !self.show_hidden && item.complete {
                    continue;
                }
                rows.push(new_row);
            }
        }
        let widths = [
            Constraint::Length(4),
            Constraint::Length(widths.name as u16),
            Constraint::Length(widths.date as u16),
            Constraint::Length(widths.repeats_every as u16),
            Constraint::Percentage(100),
        ];
        let t = Table::new(rows)
            .header(header)
            .widths(&widths)
            .column_spacing(3);
        f.render_widget(t, rects[0]);
    }
}
