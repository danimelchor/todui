use crate::app::all_tasks::AllTasksPage;
use crate::app::{App, AppPage};
use crate::utils::date_to_str;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};

pub fn render<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    terminal.draw(|f| ui(f, &page))?;

    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('j') => page.next(),
            KeyCode::Char('k') => page.prev(),
            KeyCode::Char('x') => page.toggle_selected(),
            KeyCode::Char('h') => page.toggle_hidden(),
            KeyCode::Char('d') => page.delete_selected(),
            KeyCode::Char('n') => app.current_page = AppPage::NewTask,
            _ => {}
        }
    }

    Ok(false)
}

fn ui<B: Backend>(f: &mut Frame<B>, page: &AllTasksPage) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().fg(Color::White).bg(Color::Black);
    let completed_style = Style::default().fg(Color::DarkGray);
    let header_cells = ["Status", "Name", "Date", "Repeats every", "Description"]
        .iter()
        .map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        });
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    // Rows
    let mut rows: Vec<Row> = vec![];
    let mut current_idx = 0;
    for group in page.groups() {
        for (item_idx, item) in group.iter().enumerate() {
            let x = match item.completed {
                true => "[x]",
                false => "[ ]",
            };
            let cells = vec![
                Cell::from(x),
                Cell::from(item.name.clone()),
                Cell::from(date_to_str(&item.date)),
                Cell::from(item.repeats.to_string()),
                Cell::from(item.description.clone().unwrap_or("".to_string())),
            ];

            let style = match item.completed {
                true => completed_style,
                false => Style::default(),
            };

            let style = match current_idx == page.current_idx {
                true => selected_style,
                false => style,
            };
            current_idx += 1;

            let mut new_row = Row::new(cells).height(1).style(style);

            if item_idx == group.len() - 1 {
                new_row = new_row.bottom_margin(1);
            }

            if !page.show_hidden && item.completed {
                continue;
            }
            rows.push(new_row);
        }
    }
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Todos"))
        .highlight_style(selected_style)
        .highlight_symbol("> ")
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
        ]);
    f.render_widget(t, rects[0]);
}
