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
        let all_issues =
            cacher::read_all_unclosed_issue_of_source_type_to_mem(&cache_path, "local")
                .expect("Unable to load gitlab issues from cache into gui");
        let mut all_issue_titles: Vec<Vec<String>> = vec![];
        for issue in all_issues {
            all_issue_titles.push(vec![issue.uuid.to_string(), issue.title]);
        }

        IssueTable {
            state: TableState::default(),
            items: all_issue_titles,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
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

    pub fn previous(&mut self) {
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

    pub fn refresh_with_issue_type(&mut self, cache_path: &str, issue_type: &str) {
        let all_issues =
            cacher::read_all_unclosed_issue_of_source_type_to_mem(&cache_path, issue_type).expect(
                &format!("Unable to load {} from cache into gui", issue_type),
            );
        let mut all_issue_titles: Vec<Vec<String>> = vec![];
        for issue in all_issues {
            all_issue_titles.push(vec![issue.uuid.to_string(), issue.title]);
        }

        let i = match self.state.selected() {
            Some(i) => {
                self.items.clear();
                let mut things_to_add = all_issue_titles;
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
        .widths(&[Constraint::Percentage(30), Constraint::Percentage(70)])
}
