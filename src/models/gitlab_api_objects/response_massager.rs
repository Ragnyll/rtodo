use chrono::{DateTime, Utc};
use std::fmt;

/// A GitlabIssue massaged into the "useful" fields
pub struct SimplifiedGitlabIssue {
    id: i32,
    project: Project,
    title: String,
    description: Option<String>,
    state: IssueState,
    created_dt: DateTime<Utc>,
    updated_dt: DateTime<Utc>,
    assignees: Vec<User>,
}

impl SimplifiedGitlabIssue {
    /// A parameterized constructor for a SimplifiedGitlabIssue
    pub fn new(id: i32, project: Project, title: &str, description: &str, state: IssueState, created_dt: DateTime<Utc>, updated_dt: DateTime<Utc>, assignees: Vec<User>) -> SimplifiedGitlabIssue {
        SimplifiedGitlabIssue {
            id: id,
            project: project,
            title: String::from(title),
            description: Some(String::from(description)),
            state: state,
            created_dt: created_dt,
            updated_dt: updated_dt,
            assignees: assignees,
        }
    }
}

/// A GitlabProject massaged into the "useful" fields
pub struct Project {
    id: i32,
    title: String,
    description: String,
    web_url: String,
    owner: String,
}

/// A GitlabProject massaged into the "useful" fields
pub struct User {
    id: i32,
    username: String,
    email: Option<String>,
}

/// The valid states a gitlab issue can be in
pub enum IssueState {
    Open,
    InProgress,
    Blocked,
    Closed,
}

/// For all the supplied issues finds the corresponding project information and gitlab user to
/// create a SimplifiedGitlabIssue issue from
pub fn aggregate_issues(gitlab_users: Vec<String>, gitlab_projects: Vec<String>, gitlab_issues: Vec<String>) -> Result<Vec<SimplifiedGitlabIssue>, AggregationError> {
    let simplified_issues: Vec<SimplifiedGitlabIssue> = vec!();
    Ok(simplified_issues)
}


#[derive(Debug)]
pub struct AggregationError {
    details: String,
}

impl AggregationError {
    fn new(msg: &str) -> AggregationError {
        AggregationError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AggregationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

// impl From<reqwest::Error> for AggregationError {
    // fn from(err: reqwest::Error) -> Self {
        // AggregationError::new(&err.to_string())
    // }
// }
