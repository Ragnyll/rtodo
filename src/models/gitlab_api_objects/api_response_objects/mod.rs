pub mod gitlab_issue;
pub mod gitlab_issue_components;
pub use self::gitlab_issue::GitlabIssue;
pub use self::gitlab_issue_components::Assignee;

pub mod gitlab_project;
pub mod gitlab_project_components;
pub use self::gitlab_project::GitlabProject;
pub use self::gitlab_project_components::Namespace;

pub mod gitlab_user;
pub use self::gitlab_user::GitlabUser;
