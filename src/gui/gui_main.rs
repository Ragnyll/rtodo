use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Block, Borders, Tabs};
use tui::layout::{Layout, Constraint, Direction};
use tui::text::{Span, Spans};
use tui::style::{Style, Color};
use tui::symbols::DOT;
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

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

pub fn display() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
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
    }
}
