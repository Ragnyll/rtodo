use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::layout::{Layout, Constraint, Direction};

use crate::conf::conf::Conf;
use super::components;
use super::events::events::{Event, Events};

/// Currently only optimized for 1/4 screen
pub fn display(conf: &Conf) -> Result<(), Box<dyn std::error::Error + '_>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = components::app::App::new(conf);
    let events = Events::new();
    let mut table = components::issue_table::IssueTable::new();

    loop {
        terminal.draw(|f| {
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
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => break,
                Key::Char('j') => table.next(),
                Key::Char('k') => table.previous(),
                Key::Char('h') => app.tabs.next(),
                Key::Char('l') => app.tabs.previous(),
                _ => {}
            }
        }
    }
    Ok(())
}
