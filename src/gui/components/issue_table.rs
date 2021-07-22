use tui::widgets::{Block, Borders, Cell, Row, Table, TableState};
use tui::layout::Constraint;
use tui::style::{Style, Modifier};

use crate::cache_ops::cacher;

pub struct IssueTable {
    pub state: TableState,
    pub items: Vec<Vec<String>>,
}


impl IssueTable {
    pub fn new(cache_path: &str) -> IssueTable {
        let all_issues = cacher::read_all_unclosed_gitlab_issues_to_mem(&cache_path).expect("Unable to load gitlab issues from cache into gui");
        let mut all_issue_titles: Vec<Vec<String>> = vec!();
        for issue in all_issues {
            all_issue_titles.push(vec![issue.uuid.to_string(), String::from(&issue.title[0..5])]);
        }


        IssueTable {
            state: TableState::default(),
            items: all_issue_titles,
        }
    }

    pub fn next(&mut self, issue_type: &str) {
        let i = match self.state.selected() {
            Some(i) => {
                // let mut things_to_add = vec![vec![String::from("Row11"), String::from("Row12"), String::from("Row13")]];
                // self.items.append(&mut things_to_add);
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
