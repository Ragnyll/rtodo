use super::gitlab_issue_components::assignee::Assignee;
use super::gitlab_issue_components::author::Author;
use super::gitlab_issue_components::closed_by::ClosedBy;
use super::gitlab_issue_components::time_stats::TimeStats;
use super::gitlab_issue_components::task_completion_status::TaskCompletionStatus;
use super::gitlab_issue_components::links::Links;
use super::gitlab_issue_components::references::References;

#[derive(Deserialize, Debug)]
pub struct GitlabIssue {
    id: i32,
    iid: i32,
    project_id: i32,
    title: Option<String>,
    description: Option<String>,
    state: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    closed_at: Option<String>,
    closed_by: Option<ClosedBy>,
    labels: Vec<String>,
    milestone: Option<String>,
    assignees: Option<Vec<Assignee>>,
    author: Author,
    #[serde(rename = "type")]
    type_field: Option<String>,
    assignee: Option<Assignee>,
    user_notes_count: i32,
    merge_requests_count: i32,
    upvotes: i32,
    downvotes: i32,
    due_date: Option<String>,
    confidential: bool,
    discussion_locked: Option<String>,
    issue_type: Option<String>,
    web_url: Option<String>,
    time_stats: TimeStats,
    task_completion_status: TaskCompletionStatus,
    weight: Option<String>,
    blocking_issues_count: i32,
    has_tasks: bool,
    #[serde(rename = "_links")]
    links: Links,
    references: References,
    moved_to_id: Option<String>,
    service_desk_reply_to: Option<String>,
    health_status: Option<String>,
}
