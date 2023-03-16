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
mod delete_task_page;
mod task_page;

use all_tasks_page::AllTasksPage;
use delete_task_page::DeleteTaskPage;
use task_page::TaskPage;

#[macro_export]
macro_rules! key {
    ($keybind:expr, $color:expr) => {{
        let keybind = KeyBindings::key_to_str(&$keybind);
        let keybind = format!("'{}'", keybind);
        Span::styled(keybind, Style::default().fg($color))
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
    DeleteTask,
}

#[derive(Eq, PartialEq)]
pub enum InputMode {
    Insert,
    Normal,
}

pub trait Page<B: Backend> {
    fn ui(&self, f: &mut Frame<B>, area: Rect, focused: bool);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<()> {
    let app = Rc::new(RefCell::new(app));
    let mut all_tasks_page = AllTasksPage::new(Rc::clone(&app));
    let mut task_page = TaskPage::new(Rc::clone(&app));
    let mut current_page = UIPage::AllTasks;
    let mut delete_task_page = None;

    loop {
        terminal.draw(|f| {
            render_app(
                f,
                &mut all_tasks_page,
                &mut task_page,
                &mut delete_task_page,
                &current_page,
            )
        })?;
        let keybindings = &app.borrow().settings.keybindings.clone();

        if let Event::Key(key) = event::read()? {
            let code = key.code;
            match current_page {
                UIPage::AllTasks => match code {
                    _ if code == keybindings.quit => break,
                    _ if code == keybindings.down => {
                        all_tasks_page.next();
                        if let Some(task_id) = all_tasks_page.current_id {
                            task_page = TaskPage::new_from_task(Rc::clone(&app), task_id);
                        }
                    }
                    _ if code == keybindings.up => {
                        all_tasks_page.prev();
                        if let Some(task_id) = all_tasks_page.current_id {
                            task_page = TaskPage::new_from_task(Rc::clone(&app), task_id);
                        }
                    }
                    _ if code == keybindings.complete_task => {
                        all_tasks_page.toggle_selected();
                    }
                    _ if code == keybindings.toggle_completed_tasks => {
                        all_tasks_page.toggle_hidden()
                    }
                    _ if code == keybindings.delete_task => {
                        if let Some(task_id) = all_tasks_page.current_id {
                            delete_task_page = Some(DeleteTaskPage::new(Rc::clone(&app), task_id));
                            current_page = UIPage::DeleteTask;
                        }
                    }
                    _ if code == keybindings.open_link => all_tasks_page.open_selected_link(),
                    _ if code == keybindings.new_task => {
                        current_page = UIPage::NewTask;
                        task_page = TaskPage::new(Rc::clone(&app));
                    }
                    _ if code == keybindings.edit_task => {
                        if all_tasks_page.current_id.is_some() {
                            current_page = UIPage::EditTask;
                        }
                    }
                    _ if code == keybindings.next_group => {
                        all_tasks_page.next_group();
                    }
                    _ if code == keybindings.prev_group => {
                        all_tasks_page.prev_group();
                    }
                    _ => {}
                },
                UIPage::DeleteTask => {
                    let dtp = delete_task_page.as_mut().unwrap();
                    match dtp.input_mode {
                        InputMode::Normal => match key.code {
                            _ if code == keybindings.quit => break,
                            _ if code == keybindings.enter_insert_mode => {
                                dtp.input_mode = InputMode::Insert;
                            }
                            _ if code == keybindings.go_back => {
                                current_page = UIPage::AllTasks;
                            }
                            _ if code == keybindings.save_changes => {
                                if dtp.submit() {
                                    all_tasks_page.ensure_group_exists();
                                    current_page = UIPage::AllTasks;
                                    delete_task_page = None;
                                }
                            }
                            _ => {}
                        },
                        InputMode::Insert => match key.code {
                            _ if code == keybindings.enter_normal_mode => {
                                dtp.input_mode = InputMode::Normal;
                            }
                            _ if code == keybindings.save_changes => {
                                if dtp.submit() {
                                    all_tasks_page.ensure_group_exists();
                                    current_page = UIPage::AllTasks;
                                    delete_task_page = None;
                                }
                            }
                            KeyCode::Char(c) => dtp.add_char(c),
                            KeyCode::Backspace => dtp.remove_char(),
                            _ => {}
                        },
                    }
                }
                UIPage::NewTask | UIPage::EditTask => match task_page.input_mode {
                    InputMode::Normal => match key.code {
                        _ if code == keybindings.down => task_page.next_field(),
                        _ if code == keybindings.up => task_page.prev_field(),
                        _ if code == keybindings.quit => break,
                        _ if code == keybindings.enter_insert_mode => {
                            task_page.input_mode = InputMode::Insert;
                        }
                        _ if code == keybindings.go_back => {
                            current_page = UIPage::AllTasks;
                        }
                        _ if code == keybindings.save_changes => {
                            if task_page.submit() {
                                all_tasks_page.ensure_group_exists();
                                current_page = UIPage::AllTasks;
                            }
                        }
                        _ => {}
                    },
                    InputMode::Insert => match key.code {
                        _ if code == keybindings.enter_normal_mode => {
                            task_page.input_mode = InputMode::Normal;
                        }
                        _ if code == keybindings.save_changes => {
                            if task_page.submit() {
                                all_tasks_page.ensure_group_exists();
                                current_page = UIPage::AllTasks;
                            }
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
    delete_task_page: &mut Option<DeleteTaskPage>,
    current_page: &UIPage,
) {
    let constraints = match (current_page, all_tasks_page.current_id) {
        (UIPage::AllTasks | UIPage::EditTask, Some(_)) => {
            [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref()
        }
        _ => [Constraint::Percentage(100)].as_ref(),
    };
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(f.size());

    match current_page {
        UIPage::NewTask => {
            task_page.ui(f, chunks[0], true);
        }
        UIPage::DeleteTask => {
            delete_task_page.as_mut().unwrap().ui(f, chunks[0], true);
        }
        _ => match all_tasks_page.current_id {
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
