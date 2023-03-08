use crate::app::{all_tasks::AllTasksPage, new_task::NewTaskPage, App, AppPage};
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, stdout};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod all_tasks;
mod new_task;

pub fn start_ui(app: &mut App) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    run_app(&mut terminal, app)?;
    app.save_state();

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        let quit = match app.current_page {
            AppPage::AllTasks => {
                let mut page = AllTasksPage::new(app);
                all_tasks::render(terminal, app)?
            }
            AppPage::NewTask => {
                let mut page = NewTaskPage::new(app);
                new_task::render(terminal, app)?
            }
        };

        if quit {
            break;
        }
    }

    Ok(())
}
