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

                        // Check that there are still visible tasks in group
                        let any = all_tasks_page
                            .visible_tasks()
                            .iter()
                            .any(|t| t.group == all_tasks_page.get_current_group());
                        if !any {
                            all_tasks_page.set_group(None);
                        }
                    }
                    _ if code == keybindings.toggle_completed_tasks => {
                        all_tasks_page.toggle_hidden()
                    }
                    _ if code == keybindings.delete_task => {
                        all_tasks_page.delete_selected();

                        // Check that there are still visible tasks in group
                        let any = all_tasks_page
                            .visible_tasks()
                            .iter()
                            .any(|t| t.group == all_tasks_page.get_current_group());
                        if !any {
                            all_tasks_page.set_group(None);
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
                UIPage::NewTask | UIPage::EditTask => match task_page.input_mode {
                    NewTaskInputMode::Normal => match key.code {
                        _ if code == keybindings.down => task_page.next_field(),
                        _ if code == keybindings.up => task_page.prev_field(),
                        _ if code == keybindings.quit => break,
                        _ if code == keybindings.enter_insert_mode => {
                            task_page.input_mode = NewTaskInputMode::Insert;
                        }
                        _ if code == keybindings.go_back => {
                            current_page = UIPage::AllTasks;
                        }
                        _ if code == keybindings.save_changes => {
                            let mut app = app.borrow_mut();
                            let settings = &app.settings;
                            let form_result = task_page.task_form.submit(settings);
                            match form_result {
                                Ok(new_task) => {
                                    if let Some(task_id) = task_page.editing_task {
                                        app.delete_task(task_id);
                                    }
                                    app.add_task(new_task.clone());

                                    // Drop app since we need it mutable in the next line
                                    drop(app);
                                    if new_task.group != all_tasks_page.get_current_group() {
                                        all_tasks_page.set_group(new_task.group.clone());
                                    }
                                    current_page = UIPage::AllTasks;
                                }
                                Err(e) => {
                                    task_page.error = Some(e.to_string());
                                }
                            }
                        }
                        _ => {}
                    },
                    NewTaskInputMode::Insert => match key.code {
                        _ if code == keybindings.enter_normal_mode => {
                            task_page.input_mode = NewTaskInputMode::Normal;
                        }
                        _ if code == keybindings.save_changes => {
                            // DUPLICATED - TODO: refactor
                            let mut app = task_page.app.borrow_mut();
                            let settings = &app.settings;
                            let form_result = task_page.task_form.submit(settings);
                            match form_result {
                                Ok(new_task) => {
                                    if let Some(task_id) = task_page.editing_task {
                                        app.delete_task(task_id);
                                    }
                                    app.add_task(new_task.clone());

                                    // Drop app since we need it mutable in the next line
                                    drop(app);
                                    if new_task.group != all_tasks_page.get_current_group() {
                                        all_tasks_page.set_group(new_task.group.clone());
                                    }
                                    current_page = UIPage::AllTasks;
                                }
                                Err(e) => {
                                    task_page.error = Some(e.to_string());
                                }
                            }
                            // END DUPLICATED
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

    // If no task is selected, display all_tasks page at 100%
    // If a task is selected, display all_tasks page at 50% and edit_task page at 50%
    // If edit_task page is selected, display edit_task_page at 100%

    match current_page {
        UIPage::NewTask => {
            task_page.ui(f, chunks[0], true);
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
