use crate::app::new_task::{NewTaskInputMode, NewTaskPage, TaskForm};
use crate::app::{App, AppPage};
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

pub fn render<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    terminal.draw(|f| ui(f, &page))?;

    if let Event::Key(key) = event::read()? {
        match page.input_mode {
            NewTaskInputMode::Normal => match key.code {
                KeyCode::Char('j') => page.next_field(),
                KeyCode::Char('k') => page.prev_field(),
                KeyCode::Char('q') => {
                    page.input_mode = NewTaskInputMode::Normal;
                    page.task_form = TaskForm::new();
                    return Ok(true);
                }
                KeyCode::Char('i') => {
                    page.input_mode = NewTaskInputMode::Editing;
                }
                KeyCode::Char('b') => {
                    app.current_page = AppPage::AllTasks;
                }
                KeyCode::Enter => page.submit(),
                _ => {}
            },
            _ => match key.code {
                KeyCode::Esc => {
                    page.input_mode = NewTaskInputMode::Normal;
                }
                KeyCode::Char(c) => page.add_char(c),
                KeyCode::Backspace => page.remove_char(),
                _ => {}
            },
        }
    }
    Ok(false)
}

fn border_style(page: &NewTaskPage, idx: usize) -> Style {
    if page.current_idx == idx && page.input_mode == NewTaskInputMode::Editing {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, page: &NewTaskPage) {
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
            ]
            .as_ref(),
        )
        .split(f.size());

    // Keybinds description paragraph
    let keybinds = Paragraph::new(
        "Press 'i' to enter input mode, 'q' to quit, 'j' and 'k' to move up and down, 'Enter' to submit, 'Esc' to exit input mode, and 'b' to go back to the main screen"
    ).alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(keybinds, chunks[0]);

    // Name
    let curr_text = page.task_form.name.clone();
    let input = Paragraph::new(curr_text.as_ref())
        .style(border_style(&page, 0))
        .block(Block::default().borders(Borders::ALL).title("Name"));
    f.render_widget(input, chunks[1]);

    // Date
    let curr_text = page.task_form.date.clone();
    let input = Paragraph::new(curr_text.as_ref())
        .style(border_style(&page, 1))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Date (YYYY-MM-DD)"),
        );
    f.render_widget(input, chunks[2]);

    // Repeats
    let curr_text = page.task_form.repeats.to_string();
    let input = Paragraph::new(curr_text.as_ref())
        .style(border_style(&page, 2))
        .block(Block::default().borders(Borders::ALL).title(
            "Repeats (Never | Daily | Weekly | Monthly | Yearly | Mon,Tue,Wed,Thu,Fri,Sat,Sun)",
        ));
    f.render_widget(input, chunks[3]);

    // Description
    let curr_text = page.task_form.description.clone();
    let input = Paragraph::new(curr_text.as_ref())
        .style(border_style(&page, 3))
        .block(Block::default().borders(Borders::ALL).title("Description"));
    f.render_widget(input, chunks[4]);

    // Place cursor
    match page.current_idx {
        0 => f.set_cursor(
            chunks[1].x + page.task_form.name.width() as u16 + 1,
            chunks[1].y + 1,
        ),
        1 => f.set_cursor(
            chunks[2].x + page.task_form.date.width() as u16 + 1,
            chunks[2].y + 1,
        ),
        2 => f.set_cursor(
            chunks[3].x + page.task_form.repeats.width() as u16 + 1,
            chunks[3].y + 1,
        ),
        3 => f.set_cursor(
            chunks[4].x + page.task_form.description.width() as u16 + 1,
            chunks[4].y + 1,
        ),
        _ => {}
    }

    // Error message
    if let Some(error) = &page.error {
        let error = Paragraph::new(error.as_ref())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error, chunks[4]);
    }
}
