use std::fmt;
use super::api_response_objects::{GitlabProject, GitlabUser, GitlabIssue};
use super::api_response_objects::gitlab_project_components::Namespace;
use super::api_response_objects::gitlab_issue_components::Assignee;

/// A GitlabIssue massaged into the "useful" fields
pub struct SimplifiedGitlabIssue {
    id: i32,
    project: Project,
    title: String,
    description: Option<String>,
    state: IssueState,
    assignee: Option<User>,
}

impl SimplifiedGitlabIssue {
    /// A parameterized constructor for a SimplifiedGitlabIssue
    pub fn new(
        id: i32,
        project: Project,
        title: &str,
        description: Option<String>,
        state: IssueState,
        assignee: Option<User>,
    ) -> SimplifiedGitlabIssue {
        SimplifiedGitlabIssue {
            id: id,
            project: project,
            title: String::from(title),
            description: description,
            state: state,
            assignee: assignee,
        }
    }
}

/// A GitlabProject massaged into the "useful" fields
pub struct Project {
    id: i32,
    title: String,
    description: Option<String>,
    web_url: String,
    owner: Owner,
}

// TODO: find the other owner types (namespaces)
pub enum OwnerType {
    User,
}

pub struct Owner {
    id: i32,
    owner_type: OwnerType,
}

impl Owner {
    pub fn new(id: i32, owner_type: OwnerType) -> Owner {
        Owner {
            id: id,
            owner_type: owner_type,
        }
    }
}

impl Project {
    pub fn new(
        id: i32,
        title: &str,
        description: Option<String>,
        web_url: &str,
        owner: Owner,
    ) -> Project {
        Project {
            id: id,
            title: String::from(title),
            description: description,
            web_url: String::from(web_url),
            owner: owner,
        }
    }
}
/// A GitlabProject massaged into the "useful" fields
pub struct User {
    id: i32,
    username: String,
}

impl User {
    pub fn new(id: i32, username: &str) -> User {
        User {
            id: id,
            username: String::from(username),
        }
    }
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
pub fn aggregate_issues(
    gitlab_users: Vec<GitlabUser>,
    gitlab_projects: Vec<GitlabProject>,
    gitlab_issues: Vec<GitlabIssue>,
) -> Result<Vec<SimplifiedGitlabIssue>, AggregationError> {
    // TODO: Implement a cache for the users and projects
    let mut simplified_issues: Vec<SimplifiedGitlabIssue> = vec![];
    // TODO: make this async
    for issue in gitlab_issues.into_iter() {
        // find matching project
        let corresponding_project = gitlab_projects
            .iter()
            .find(|&project| project.get_id() == issue.get_project_id())
            .ok_or(AggregationError::new(&format!(
                "issue_id {} is invalid as there is not a corresponding project with id {}",
                issue.get_id(),
                issue.get_project_id()
            )))?;

        let owner = convert_namespace_to_owner(corresponding_project.get_namespace());
        let project = convert_gitlab_project_to_project(corresponding_project, owner)?;
        // find matching assignees

        let assignee: Option<User> = match issue.get_assignee() {
            Some(a) => {
                Some(convert_gitlab_assignee_to_user(a)?)
            },
            // Its ok for an issue not to have an assignee
            None => None
        };
        simplified_issues.push(SimplifiedGitlabIssue::new(
            issue.get_id(),
            project,
            issue.get_title().as_ref().ok_or(ConversionError::new(&format!(
                "issue_id {} is invalid as there is no title",
                issue.get_id())))?,
             issue.get_description().clone(),
            map_gitlab_issue_state_to_issue_state(issue.get_state().clone())?,
            assignee,
        ));
    }
    Ok(simplified_issues)
}

fn map_gitlab_issue_state_to_issue_state(gitlab_issue_state: Option<String>) -> Result<IssueState, ConversionError> {

    let closed = String::from("closed");
    let open = String::from("open");
    match gitlab_issue_state {
        None => Err(ConversionError::new("Unable to map gitlab_issue_state with None to IssueState")),
        Some(s) => match String::from(s) {
            closed => Ok(IssueState::Closed),
            open => Ok(IssueState::Open),
        }

    }
}

// TODO: replace with logic to convert to correct owner type
/// Converts a GitlabProject.Namespace to an Owner
fn convert_namespace_to_owner(gitlab_namespace: &Namespace) -> Owner {
    Owner::new(gitlab_namespace.get_id(), OwnerType::User)
}

fn convert_gitlab_assignee_to_user(gitlab_assignee: &Assignee) -> Result<User, ConversionError> {
    Ok(User::new(
        gitlab_assignee.get_id(),
        gitlab_assignee
            .get_username()
            .as_ref()
            .ok_or(ConversionError::new(&format!(
                "The Assignee {} is invalid as it does not have a username",
                gitlab_assignee.get_id()
            )))?
    ))
}

fn convert_gitlab_project_to_project(
    gitlab_project: &GitlabProject,
    owner: Owner,
) -> Result<Project, ConversionError> {
    Ok(Project::new(
        gitlab_project.get_id(),
        &gitlab_project
            .get_name()
            .as_ref()
            .ok_or(ConversionError::new(&format!(
                "The project {} is invalid as it does not have a name",
                &gitlab_project.get_id()
            )))?,
        gitlab_project.get_description().clone(),
        gitlab_project
            .get_web_url()
            .as_ref()
            .ok_or(ConversionError::new(&format!(
                "The project {} is invalid as it does not have a web_url",
                &gitlab_project.get_id()
            )))?,
        owner,
    ))
}

#[derive(Debug)]
pub struct AggregationError {
    details: String,
}

impl AggregationError {
    #[allow(dead_code)]
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


impl From<ConversionError> for AggregationError {
    fn from(err: ConversionError) -> Self {
        AggregationError::new(&err.to_string())
    }
}

#[derive(Debug)]
pub struct ConversionError {
    details: String,
}

impl ConversionError {
    #[allow(dead_code)]
    fn new(msg: &str) -> ConversionError {
        ConversionError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}


