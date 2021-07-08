use tui::widgets::{Block, Borders, Tabs};
use tui::text::Spans;
use tui::style::{Style, Color};

use crate::conf::conf::Conf;

pub fn create_tabs(conf: &Conf) -> Tabs<'static> {
    let tab_titles = conf.get_todo_types().iter().cloned().map(Spans::from).collect();
    Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Todo Types"))
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().bg(Color::Black))
}
