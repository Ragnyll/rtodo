use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use tui::layout::Constraint;
use tui::style::{Style, Modifier};

use crate::cache_ops::cacher;

pub struct IssueTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}


impl IssueTable {
    pub fn new() -> IssueTable {
        IssueTable {
            state: TableState::default(),
            items: vec![
                vec![String::from("Row21"), String::from("Row22"), String::from("Row23")],
                vec![String::from("Row31"), String::from("Row32"), String::from("Row33")],
                vec![String::from("Row41"), String::from("Row42"), String::from("Row43")],
                vec![String::from("Row51"), String::from("Row52"), String::from("Row53")],
                vec![String::from("Row61"), String::from("Row62"), String::from("Row63")],
                vec![String::from("Row71"), String::from("Row72"), String::from("Row73")],
                vec![String::from("Row81"), String::from("Row82"), String::from("Row83")],
                vec![String::from("Row91"), String::from("Row92"), String::from("Row93")],
            ],
        }
    }
    pub fn next(&mut self, issue_type: &str) {
        let i = match self.state.selected() {
            Some(i) => {
                let mut things_to_add = vec![vec![String::from("Row11"), String::from("Row12"), String::from("Row13")]];
                self.items.append(&mut things_to_add);
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, issue_type: &str) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn create_table(table: &IssueTable) -> Table<'static> {
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
    Table::new(rows)
    .block(Block::default().borders(Borders::ALL))
    .highlight_style(selected_style)
    .highlight_symbol("> ")
    .widths(&[
        Constraint::Percentage(100)
    ])
}
