use crate::app::App;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

mod all_tasks_page;
mod task_page;

use all_tasks_page::AllTasksPage;
use task_page::TaskPage;

use task_page::NewTaskInputMode;

#[macro_export]
macro_rules! key {
    ($keybind:expr) => {{
        let keybind = format!("'{}'", $keybind);
        Span::styled(keybind, Style::default().fg(Color::LightBlue))
    }};
}

pub fn start_ui(app: App) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    run_app(&mut terminal, app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[derive(Eq, PartialEq)]
pub enum UIPage {
    AllTasks,
    NewTask,
    EditTask,
}

pub trait Page<B: Backend> {
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<()> {
    let app = Rc::new(RefCell::new(app));
    let mut all_tasks_page = AllTasksPage::new(Rc::clone(&app));
    let mut task_page = TaskPage::new(Rc::clone(&app));
    let mut current_page = UIPage::AllTasks;

    loop {
        terminal.draw(|f| render_app(f, &mut all_tasks_page, &mut task_page, &current_page))?;

        if let Event::Key(key) = event::read()? {
            match current_page {
                UIPage::AllTasks => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') => {
                        all_tasks_page.next();
                        let current_idx = all_tasks_page.current_idx.unwrap();
                        let task_id = app.borrow().tasks[current_idx].id.unwrap();
                        task_page = TaskPage::new_from_task(Rc::clone(&app), task_id);
                    }
                    KeyCode::Char('k') => {
                        all_tasks_page.prev();
                        let current_idx = all_tasks_page.current_idx.unwrap();
                        let task_id = app.borrow().tasks[current_idx].id.unwrap();
                        task_page = TaskPage::new_from_task(Rc::clone(&app), task_id);
                    }
                    KeyCode::Char('x') => all_tasks_page.toggle_selected(),
                    KeyCode::Char('h') => all_tasks_page.toggle_hidden(),
                    KeyCode::Char('d') => all_tasks_page.delete_selected(),
                    KeyCode::Enter => all_tasks_page.open_selected_link(),
                    KeyCode::Char('n') => {
                        current_page = UIPage::NewTask;
                        task_page = TaskPage::new(Rc::clone(&app));
                    }
                    KeyCode::Char('e') => {
                        if let Some(_) = all_tasks_page.current_idx {
                            current_page = UIPage::EditTask;
                        }
                    }
                    _ => {}
                },
                UIPage::NewTask | UIPage::EditTask => match task_page.input_mode {
                    NewTaskInputMode::Normal => match key.code {
                        KeyCode::Char('j') => task_page.next_field(),
                        KeyCode::Char('k') => task_page.prev_field(),
                        KeyCode::Char('q') => break,
                        KeyCode::Char('i') => {
                            task_page.input_mode = NewTaskInputMode::Editing;
                        }
                        KeyCode::Char('b') => {
                            current_page = UIPage::AllTasks;
                        }
                        KeyCode::Enter => {
                            let mut app = task_page.app.borrow_mut();
                            let settings = &app.settings;
                            let form_result = task_page.task_form.submit(&settings);
                            match form_result {
                                Ok(new_taks) => {
                                    if let Some(task_id) = task_page.editing_task {
                                        app.delete_task(task_id);
                                    }
                                    app.add_task(new_taks);
                                    current_page = UIPage::AllTasks;
                                }
                                Err(e) => {
                                    task_page.error = Some(e.to_string());
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => match key.code {
                        KeyCode::Esc => {
                            task_page.input_mode = NewTaskInputMode::Normal;
                        }
                        KeyCode::Char(c) => task_page.add_char(c),
                        KeyCode::Backspace => task_page.remove_char(),
                        _ => {}
                    },
                },
            }
        }
    }

    Ok(())
}

fn render_app<B: Backend>(
    f: &mut Frame<B>,
    all_tasks_page: &mut AllTasksPage,
    task_page: &mut TaskPage,
    current_page: &UIPage,
) {
    let constraints = match (current_page, all_tasks_page.current_idx) {
        (UIPage::AllTasks | UIPage::EditTask, Some(_)) => [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
        _ => [Constraint::Percentage(100)].as_ref(),
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.size());

    // If no task is selected, display all_tasks page at 100%
    // If a task is selected, display all_tasks page at 50% and edit_task page at 50%
    // If edit_task page is selected, display edit_task_page at 100%

    match current_page {
        UIPage::NewTask => {
            task_page.ui(f, chunks[0], true);
        }
        _ => match all_tasks_page.current_idx {
            Some(_) => {
                all_tasks_page.ui(f, chunks[0], current_page == &UIPage::AllTasks);
                task_page.ui(f, chunks[1], current_page == &UIPage::EditTask);
            }
            None => {
                all_tasks_page.ui(f, chunks[0], true);
            }
        },
    }
}
