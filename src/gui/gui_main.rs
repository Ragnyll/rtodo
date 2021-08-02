use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Layout, Constraint, Direction};

use tui::{
    widgets::{Block, Borders, Clear},
    layout::Rect,
};
use crate::conf::conf::Conf;
use super::components;
use super::events::events::{Event, Events};

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

/// Currently only optimized for 1/4 screen
/// TODO: figure out the lifetime issue
pub fn display(conf: &Conf, cache_path: String) -> Result<(), Box<dyn std::error::Error + '_>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    let mut app = components::app::App::new(conf);
    let events = Events::new();
    let mut table = components::issue_table::IssueTable::new(&cache_path);
    table.next();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10),
                    Constraint::Percentage(90)].as_ref())
                .split(f.size());


            // Create objects to render
            let tabs = components::tab_bar::create_and_update_tabs(&app, conf);
            let t = components::issue_table::create_table(&table);

            // Render as widgets
            f.render_widget(tabs, chunks[0]);
            f.render_stateful_widget(t, chunks[1], &mut table.state);

            // When the popup is visible grab the currently selected issue from the table and show
            // all the details on it
            if app.popup_visible {
                // table.items[table.state.selected().unwrap()][1]
                let block = Block::default().borders(Borders::ALL);
                let area = centered_rect(70, 80, size);
                f.render_widget(Clear, area); //this clears out the background
                f.render_widget(block, area);
            }
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => break,
                Key::Char('j') => table.next(),
                Key::Char('k') => table.previous(),
                Key::Char('h') => {
                    app.tabs.previous();
                    let next_todo_type = app.tabs.get_next_todo_type().expect("could not determine current_tab_type");
                    table.refresh_with_issue_type(&cache_path, next_todo_type);
                },
                Key::Char('l') => {
                    app.tabs.next();
                    let next_todo_type = app.tabs.get_next_todo_type().expect("could not determine current_tab_type");
                    table.refresh_with_issue_type(&cache_path, next_todo_type);
                },
                Key::Char('v') => {
                    app.toggle_popup()
                },
                _ => {}
            }
        }
    }
    Ok(())
}
