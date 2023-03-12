use super::utils::wrap_text;
use crate::app::App;
use crate::task::Task;
use crate::ui::{Page, UIPage};
use crate::utils::date_to_str;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use itertools::Itertools;
use tui::widgets::BorderType;
use std::cell::RefCell;
use std::rc::Rc;
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
        AllTasksPage {
            show_hidden: true,
            current_idx: None,
            app,
        }
    }

    pub fn toggle_selected(&mut self) {
        if self.current_idx.is_none() {
            return;
        }

        self.app
            .borrow_mut()
            .toggle_completed_task(self.current_idx.unwrap());

        if !self.show_hidden {
            self.move_closest();
        }
    }

    pub fn delete_selected(&mut self) {
        if self.current_idx.is_none() {
            return;
        }

        self.app.borrow_mut().delete_task(self.current_idx.unwrap());
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
            if !self.app.borrow().tasks[i].completed {
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
            if !self.app.borrow().tasks[i].completed {
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
            if !app.tasks[i].completed {
                self.current_idx = Some(i);
                return;
            }
        }

        for i in (0..curr_idx).rev() {
            if !app.tasks[i].completed {
                self.current_idx = Some(i);
                return;
            }
        }

        self.current_idx = None;
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        if !self.show_hidden {
            self.move_closest();
        }
    }
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
                KeyCode::Char('n') => return Ok(UIPage::NewTask),
                KeyCode::Char('e') => {
                    if let Some(idx) = self.current_idx {
                        return Ok(UIPage::EditTask(idx));
                    }
                }
                _ => {}
            }
        }

        Ok(UIPage::SamePage)
    }

    fn ui(&self, f: &mut Frame<B>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(f.size());

        let header_cells = ["Done", "Name", "Date", "Repeats every", "Description"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        // Rows
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let completed_style = Style::default().fg(Color::DarkGray);
        let default_style = Style::default().fg(Color::White);

        let mut rows: Vec<Row> = vec![];
        let mut current_idx = 0;
        for group in self.groups() {
            for (item_idx, item) in group.iter().enumerate() {
                let x = match item.completed {
                    true => "󰄴",
                    false => "󰝦",
                };
                let cells = vec![
                    Cell::from(x),
                    Cell::from(wrap_text(item.name.clone(), 25)),
                    Cell::from(date_to_str(&item.date)),
                    Cell::from(item.repeats.to_string()),
                    Cell::from(wrap_text(
                        item.description.clone().unwrap_or("".to_string()),
                        35,
                    )),
                ];

                let height = vec![
                    item.name.len() / 25,
                    item.description.clone().unwrap_or("".to_string()).len() / 35,
                ]
                .iter()
                .max()
                .unwrap()
                .clone()
                    + 1;

                let style = match item.completed {
                    true => completed_style,
                    false => default_style,
                };

                let style = match self.current_idx {
                    Some(idx) if idx == current_idx => selected_style,
                    _ => style,
                };
                current_idx += 1;

                let mut new_row = Row::new(cells).height(height as u16).style(style);

                if item_idx == group.len() - 1 {
                    new_row = new_row.bottom_margin(1);
                }

                if !self.show_hidden && item.completed {
                    continue;
                }
                rows.push(new_row);
            }
        }
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Todos"))
            .widths(&[
                Constraint::Length(4),
                Constraint::Length(25),
                Constraint::Length(10),
                Constraint::Min(13),
                Constraint::Length(35),
            ])
            .column_spacing(2);
        f.render_widget(t, rects[0]);
    }
}
