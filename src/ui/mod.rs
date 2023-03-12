use crate::app::App;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

mod all_tasks;
mod new_task;
mod utils;

use new_task::NewTaskPage;
use all_tasks::AllTasksPage;
use utils::wrap_text;

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
    Quit,
    SamePage,
    AllTasks,
    NewTask,
    EditTask(usize),
}

pub trait Page<B: Backend> {
    fn render(&mut self, terminal: &mut Terminal<B>) -> Result<UIPage>;
    fn ui(&self, f: &mut Frame<B>);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> Result<()> {
    let app = Rc::new(RefCell::new(app));
    let mut curr_page: Box<dyn Page<B>> = Box::new(AllTasksPage::new(Rc::clone(&app)));

    loop {
        let new_page_type = curr_page.render(terminal)?;

        match new_page_type {
            UIPage::Quit => break,
            UIPage::AllTasks => {
                curr_page = Box::new(AllTasksPage::new(Rc::clone(&app)));
            }
            UIPage::NewTask => {
                curr_page = Box::new(NewTaskPage::new(Rc::clone(&app)));
            }
            UIPage::EditTask(task_id) => {
                curr_page = Box::new(NewTaskPage::new_from_task(Rc::clone(&app), task_id));
            }
            _ => {}
        }
    }
    

    Ok(())
}
