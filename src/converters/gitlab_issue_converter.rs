use crate::todo_issues::todo_issue::todo_issues::{
    TodoIssue, User, IssueState, Project, Owner, OwnerType, Convertable, ConversionError,
};

use crate::models::gitlab_api_objects::GitlabProject;
use crate::models::gitlab_api_objects::api_response_objects::GitlabIssue;
use crate::models::gitlab_api_objects::api_response_objects::Namespace;
use crate::models::gitlab_api_objects::api_response_objects::Assignee;

const GITLAB_ISSUE_SOURCE: &str = "GITLAB";

#[derive(Debug)]
pub struct GitlabIssueContainer {
    gitlab_issue: GitlabIssue,
    gitlab_projects: Vec<GitlabProject>,
}

impl GitlabIssueContainer {
    pub fn new(gitlab_issue: GitlabIssue, gitlab_projects: Vec<GitlabProject>) -> GitlabIssueContainer {
        GitlabIssueContainer {
            gitlab_issue: gitlab_issue,
            gitlab_projects: gitlab_projects
        }
    }
}

/// For all the supplied issues finds the corresponding project information and gitlab user to
/// create a TodoIssue issue from
/// Self is an object that contains issue, and all Projects + Users
impl Convertable for GitlabIssueContainer {
    fn convert_to_todo_issue(&self) -> Result<TodoIssue, ConversionError> {
        let corresponding_project = self
            .gitlab_projects
            .iter()
            .find(|&project| project.get_id() == self.gitlab_issue.get_project_id())
            .ok_or(ConversionError::new(&format!(
                "issue_id {} is invalid as there is not a corresponding project with id {}",
                self.gitlab_issue.get_id(),
                self.gitlab_issue.get_project_id()
            )))?;

        let owner = convert_namespace_to_owner(corresponding_project.get_namespace());
        let project = convert_gitlab_project_to_project(corresponding_project, owner)?;
        // find matching assignees

        let assignee: Option<User> = match self.gitlab_issue.get_assignee() {
            Some(a) => Some(convert_gitlab_assignee_to_user(a)?),
            // Its ok for an issue not to have an assignee
            None => None,
        };
        Ok(TodoIssue::new(
            self.gitlab_issue.get_id(),
            project,
            self.gitlab_issue
                .get_title()
                .as_ref()
                .ok_or(ConversionError::new(&format!(
                    "issue_id {} is invalid as there is no title",
                    self.gitlab_issue.get_id()
                )))?,
            self.gitlab_issue.get_description().clone(),
            map_gitlab_issue_state_to_issue_state(self.gitlab_issue.get_state().clone())?,
            GITLAB_ISSUE_SOURCE,
            assignee,
        ))
    }
}

fn map_gitlab_issue_state_to_issue_state(
    gitlab_issue_state: Option<String>,
) -> Result<IssueState, ConversionError> {
    match gitlab_issue_state {
        None => Err(ConversionError::new(
            "Unable to map gitlab_issue_state with None to IssueState",
        )),
        Some(s) => {
            if s == "opened" {
                Ok(IssueState::Open)
            } else {
                Ok(IssueState::Closed)
            }
        },
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
            )))?,
    ))
}

fn convert_gitlab_project_to_project(
    gitlab_project: &GitlabProject,
    owner: Owner,
) -> Result<Project, ConversionError> {
    // pub fn new(id: i32, title: &str, description: Option<String>, web_url: Option<String>, owner: Owner) -> Project {
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
        gitlab_project.get_web_url().clone(),
        owner,
    ))
}
