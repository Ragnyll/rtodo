use std::fmt;

pub mod todo_issues {
    /// A struct representing an issue todo
    #[derive(Debug)]
    pub struct TodoIssue {
        id: i32,
        project: Project,
        title: String,
        description: Option<String>,
        state: IssueState,
        assignee: Option<User>,
    }

    impl TodoIssue {
        pub fn new(
            id: i32,
            project: Project,
            title: &str,
            description: Option<String>,
            state: IssueState,
            assignee: Option<User>,
        ) -> TodoIssue {
            TodoIssue {
                id: id,
                project: project,
                title: String::from(title),
                description: description,
                state: state,
                assignee: assignee,
            }
        }
    }

    #[derive(Debug)]
    pub struct Project {
        id: i32,
        title: String,
        description: Option<String>,
        web_url: Option<String>,
        owner: Owner,
    }

    impl Project {
        pub fn new(
            id: i32,
            title: &str,
            description: Option<String>,
            web_url: Option<String>,
            owner: Owner,
        ) -> Project {
            Project {
                id: id,
                title: String::from(title),
                description: description,
                web_url: web_url,
                owner: owner,
            }
        }
    }

    #[derive(Debug)]
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

    // TODO: find other types
    #[derive(Debug)]
    pub enum OwnerType {
        User,
    }

    /// The valid states an issue can be in
    #[derive(Debug)]
    pub enum IssueState {
        Open,
        Closed,
    }

    #[derive(Debug)]
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

    /// A Trait used for any external issue that can convert to a TodoIssue
    pub trait Convertable {
        fn convert_to_todo_issue(&self) -> Result<TodoIssue, ConversionError>;
    }

    #[derive(Debug)]
    pub struct ConversionError {
        details: String,
    }

    impl ConversionError {
        #[allow(dead_code)]
        pub fn new(msg: &str) -> ConversionError {
            ConversionError {
                details: msg.to_string(),
            }
        }
    }

    use std::fmt;

    impl fmt::Display for ConversionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }
}
