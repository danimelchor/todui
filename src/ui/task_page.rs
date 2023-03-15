use crate::{app::App, key, task_form::TaskForm, utils};
use std::{cell::RefCell, rc::Rc};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::Page;

#[derive(PartialEq)]
pub enum NewTaskInputMode {
    Normal,
    Insert,
}

pub struct TaskPage {
    pub task_form: TaskForm,
    pub input_mode: NewTaskInputMode,
    pub editing_task: Option<usize>,
    pub current_idx: usize,
    pub num_fields: usize,
    pub error: Option<String>,
    pub app: Rc<RefCell<App>>,
}

impl TaskPage {
    pub fn new(app: Rc<RefCell<App>>) -> TaskPage {
        TaskPage {
            task_form: TaskForm::new(),
            input_mode: NewTaskInputMode::Normal,
            current_idx: 0,
            error: None,
            num_fields: 5,
            editing_task: None,
            app,
        }
    }

    pub fn new_from_task(app: Rc<RefCell<App>>, task_id: usize) -> TaskPage {
        let task = app.borrow().get_task(task_id).unwrap().clone();
        let mut task_form = TaskForm::new();

        task_form.name = task.name.to_string();
        task_form.date = utils::date_to_input_str(&task.date, &app.borrow().settings);
        task_form.repeats = task.repeats.to_string();
        task_form.description = task.description.unwrap_or("".to_string());
        task_form.url = task.url.unwrap_or("".to_string());

        TaskPage {
            task_form,
            input_mode: NewTaskInputMode::Normal,
            current_idx: 0,
            error: None,
            num_fields: 5,
            editing_task: Some(task_id),
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

    pub fn add_char(&mut self, c: char) {
        match self.current_idx {
            0 => self.task_form.name.push(c),
            1 => self.task_form.date.push(c),
            2 => self.task_form.repeats.push(c),
            3 => self.task_form.description.push(c),
            4 => self.task_form.url.push(c),
            _ => {}
        };
    }

    pub fn remove_char(&mut self) {
        match self.current_idx {
            0 => self.task_form.name.pop(),
            1 => self.task_form.date.pop(),
            2 => self.task_form.repeats.pop(),
            3 => self.task_form.description.pop(),
            4 => self.task_form.url.pop(),
            _ => None,
        };
    }

    fn border_style(&self, idx: usize) -> Style {
        if self.current_idx == idx && self.input_mode == NewTaskInputMode::Insert {
            Style::default().fg(self.get_primary_color())
        } else {
            Style::default()
        }
    }

    fn get_date_hint(&self) -> String {
        let date_hint = self
            .app
            .borrow()
            .settings
            .date_formats
            .input_date_hint
            .clone();
        let datetime_hint = self
            .app
            .borrow()
            .settings
            .date_formats
            .input_datetime_hint
            .clone();
        format!("{} or {}", date_hint, datetime_hint)
    }

    fn get_keybind_hint(&self) -> Spans {
        let color = self.get_secondary_color();
        let i = key!("i", color);
        let q = key!("q", color);
        let j = key!("j", color);
        let k = key!("k", color);
        let enter = key!("enter", color);
        let esc = key!("esc", color);
        let b = key!("b", color);

        Spans::from(vec![
            Span::raw("Press "),
            i,
            Span::raw(" to enter insert mode, "),
            q,
            Span::raw(" to quit, "),
            j,
            Span::raw(" and "),
            k,
            Span::raw(" to move up and down, "),
            enter,
            Span::raw(" to save, "),
            esc,
            Span::raw(" to exit input mode, and "),
            b,
            Span::raw(" to go back to the main screen. (*) Fields are required."),
        ])
    }

    pub fn get_primary_color(&self) -> Color {
        self.app.borrow().settings.colors.primary_color
    }

    pub fn get_secondary_color(&self) -> Color {
        self.app.borrow().settings.colors.secondary_color
    }
}

impl<B> Page<B> for TaskPage
where
    B: Backend,
{
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(area);

        // Draw border around area
        let border_style = match focused {
            true => Style::default().fg(self.get_primary_color()),
            false => Style::default(),
        };
        let border_type = match focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Task")
            .border_style(border_style)
            .border_type(border_type);
        f.render_widget(block, area);

        // Keybinds description paragraph
        let keybinds = Paragraph::new(self.get_keybind_hint())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(keybinds, chunks[0]);

        // Name
        let curr_text = self.task_form.name.clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(0))
            .block(Block::default().borders(Borders::ALL).title("Name (*)"));
        f.render_widget(input, chunks[1]);

        // Date
        let curr_text = self.task_form.date.clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(1))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Date ({})", self.get_date_hint())),
            );
        f.render_widget(input, chunks[2]);

        // Repeats
        let curr_text = self.task_form.repeats.to_string();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(2))
            .block(Block::default().borders(Borders::ALL).title(
                "Repeats (Never | Daily | Weekly | Monthly | Yearly | Mon,Tue,Wed,Thu,Fri,Sat,Sun)",
            ));
        f.render_widget(input, chunks[3]);

        // Description
        let curr_text = self.task_form.description.clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(3))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Description"),
            );
        f.render_widget(input, chunks[4]);
        
        // Description
        let curr_text = self.task_form.url.clone();
        let input = Paragraph::new(curr_text.as_ref())
            .style(self.border_style(4))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("URL"),
            );
        f.render_widget(input, chunks[5]);

        // Place cursor
        if focused {
            match self.current_idx {
                0 => f.set_cursor(
                    chunks[1].x + self.task_form.name.width() as u16 + 1,
                    chunks[1].y + 1,
                ),
                1 => f.set_cursor(
                    chunks[2].x + self.task_form.date.width() as u16 + 1,
                    chunks[2].y + 1,
                ),
                2 => f.set_cursor(
                    chunks[3].x + self.task_form.repeats.width() as u16 + 1,
                    chunks[3].y + 1,
                ),
                3 => f.set_cursor(
                    chunks[4].x + self.task_form.description.width() as u16 + 1,
                    chunks[4].y + 1,
                ),
                4 => f.set_cursor(
                    chunks[5].x + self.task_form.url.width() as u16 + 1,
                    chunks[5].y + 1,
                ),
                _ => {}
            }
        }

        // Error message
        if let Some(error) = &self.error {
            let error = Paragraph::new(error.as_ref())
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, chunks[6]);
        }
    }
}
