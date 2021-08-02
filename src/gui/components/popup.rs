use tui::layout::{Direction, Constraint, Layout};
use tui::{
    text::Spans,
    widgets::{Block, Borders, Paragraph, Wrap},
    layout::Rect,
};
use crate::cache_ops::cacher;

/// helper function to create a centered rect using up
/// certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

pub fn render_issue_popup<'a>(issue_uuid: &'a str, cache_path: &'a str) -> Paragraph<'a> {
    let block = Block::default().borders(Borders::ALL);
    // There will only ever be 1 in this array, so just get the 0 index.
    let issue = cacher::read_issue_into_memory_by_uuid(
        &cache_path,
        uuid::Uuid::parse_str(&issue_uuid).expect("unable to retrive issue from cache"),
    )
    .expect(&format!("Unable to read {} from cache", issue_uuid));
    if issue.len() != 1 {
        panic!("issue retrieved was not unique");
    }
    let issue = &issue[0];
    let title = Spans::from(format!("Title: {}", issue.title));
    let project: Spans;
    if issue.project.is_some() {
        project = Spans::from(format!(
            "Project: {}",
            issue.project.as_ref().unwrap().title
        ));
    } else {
        project = Spans::from("Project: No associated project");
    }

    let description: Spans;
    if issue.description.is_some() {
        description = Spans::from(format!(
            "Description: {}",
            issue.description.as_ref().unwrap()
        ));
    } else {
        description = Spans::from("Description: No associated description");
    }

    let assignee: Spans;
    if issue.assignee.is_some() {
        assignee = Spans::from(format!(
            "Assignee: {}",
            issue.assignee.as_ref().unwrap().username
        ));
    } else {
        assignee = Spans::from("Assignee: No associated assignee");
    }

    let text = vec![
        title,
        Spans::from(""),
        project,
        Spans::from(""),
        description,
        Spans::from(""),
        assignee,
    ];

    Paragraph::new(text).block(block).wrap(Wrap { trim: true })
}
