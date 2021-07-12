use std::io;
use termion::{event::Key, raw::IntoRawMode, screen::AlternateScreen};
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, Cell, Row, Table};
use tui::layout::{Layout, Constraint, Direction};
use tui::style::{Style, Color, Modifier};

use crate::conf::conf::Conf;
use super::components;
use super::events::events::{Event, Events};

pub struct TabsState<> {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

struct App {
    tabs: TabsState,
}




/// Currently only optimized for 1/4 screen
pub fn display(conf: &Conf) -> Result<(), Box<dyn std::error::Error + '_>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut table = components::issue_table::IssueTable::new();

    let mut app = App {
        tabs: TabsState::new(conf.get_todo_types()),
    };

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10),
                    Constraint::Percentage(90)].as_ref())
                .split(f.size());

            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let rows = table.items.iter().map(|item| {
                let height = item
                    .iter()
                    .map(|content| content.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0)
                    + 1;
                let cells = item.iter().map(|c| Cell::from(c.clone()));
                Row::new(cells).height(height as u16).bottom_margin(1)
            });
            let t = Table::new(rows)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(selected_style)
                .highlight_symbol("> ")
                .widths(&[
                    Constraint::Percentage(100)
                ]);
            f.render_stateful_widget(t, chunks[1], &mut table.state);

            let tabs = components::tab_bar::create_tabs(conf);
            let updated_tabs = tabs.select(app.tabs.index)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );

            f.render_widget(updated_tabs, chunks[0]);
            let block = Block::default().title("Issues").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
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
