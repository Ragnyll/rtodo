use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::Spans;
use tui::style::{Style, Color};

use super::events::events::{Event, Events};

fn create_tabs() -> Tabs<'static> {
    let mut issue_types = vec![];
    issue_types.push("Local");
    issue_types.push("Gitlab");
    let titles = issue_types.iter().cloned().map(Spans::from).collect();
    Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().bg(Color::Black))
}

pub fn display() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let tabs = create_tabs();
            f.render_widget(tabs, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Input(input) = events.next()? {
            match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
