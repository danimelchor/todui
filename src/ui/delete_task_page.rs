use crate::{
    app::{App, Id},
    configuration::KeyBindings,
    key,
};
use std::{cell::RefCell, rc::Rc};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::{InputMode, Page};

pub struct DeleteTaskPage {
    pub task_form: String,
    pub input_mode: InputMode,
    pub task_id: Id,
    pub error: Option<String>,
    pub app: Rc<RefCell<App>>,
}

impl DeleteTaskPage {
    pub fn new(app: Rc<RefCell<App>>, task_id: Id) -> Self {
        Self {
            task_form: String::default(),
            input_mode: InputMode::Normal,
            error: None,
            task_id,
            app,
        }
    }

    pub fn get_task_name(&self) -> String {
        self.app
            .borrow()
            .get_task(self.task_id)
            .unwrap()
            .name
            .clone()
    }

    pub fn remove_task(&self) {
        self.app.borrow_mut().delete_task(self.task_id);
    }

    pub fn add_char(&mut self, c: char) {
        self.task_form.push(c);
    }

    pub fn remove_char(&mut self) {
        self.task_form.pop();
    }

    pub fn submit(&mut self) -> bool {
        if self.task_form == self.get_task_name() {
            self.remove_task();
            true
        } else {
            self.error = Some(format!(
                "The name you entered is not the same as the task name: '{}'",
                self.get_task_name()
            ));
            false
        }
    }

    fn get_keybind_hint(&self) -> Line {
        let color = self.get_secondary_color();
        let kb = &self.app.borrow().settings.keybindings;
        let i = key!(kb.enter_insert_mode, color);
        let q = key!(kb.quit, color);
        let enter = key!(kb.save_changes, color);
        let esc = key!(kb.enter_normal_mode, color);
        let b = key!(kb.go_back, color);

        Line::from(vec![
            Span::raw("Press "),
            i,
            Span::raw(" to enter insert mode, "),
            q,
            Span::raw(" to quit, "),
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

impl Page for DeleteTaskPage {
    fn ui(&self, f: &mut Frame, area: Rect, focused: bool) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
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

        // Description
        let description = Paragraph::new(Line::from(format!(
            "To delete this task, please write down the exact name: '{}'",
            self.get_task_name()
        )))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(description, chunks[1]);

        // Name input
        let curr_text = Text::from(self.task_form.clone());
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Insert => Style::default().fg(self.get_primary_color()),
        };
        let input = Paragraph::new(curr_text)
            .style(style)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, chunks[2]);

        // Place cursor
        if focused {
            f.set_cursor_position((
                chunks[2].x + self.task_form.width() as u16 + 1,
                chunks[2].y + 1,
            ));
        }

        // Error message
        if let Some(error) = &self.error {
            let error = Paragraph::new(Text::from(error.to_owned()))
                .style(Style::default().fg(Color::Red))
                .block(Block::default().borders(Borders::ALL).title("Error"));
            f.render_widget(error, chunks[3]);
        }
    }
}
