use tui::widgets::{Block, Borders, Tabs};
use tui::text::Spans;
use tui::style::{Style, Color, Modifier};

use crate::conf::conf::Conf;
use super::app::App;

fn create_tabs(conf: &Conf) -> Tabs<'static> {
    // let mut tab_titles = vec![String::from("ALL")];
    // tab_titles.append(&mut conf.get_todo_types());
    let tab_titles = conf
        .get_todo_types()
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
    Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Todo Types"))
        .style(Style::default().fg(Color::Green))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        )
}

pub fn create_and_update_tabs(app: &App, conf: &Conf) -> Tabs<'static> {
    let tabs = create_tabs(conf);
    tabs.select(app.tabs.index).highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Black),
    )
}
