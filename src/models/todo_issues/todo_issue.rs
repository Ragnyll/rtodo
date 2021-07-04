pub mod todo_issues {
    /// A struct representing an issue todo
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TodoIssue {
        uuid: uuid::Uuid,
        source_id: Option<i32>,
        project: Option<Project>,
        title: String,
        description: Option<String>,
        state: IssueState,
        source: String,
        assignee: Option<User>,
    }

    impl TodoIssue {
        pub fn new(
            source_id: Option<i32>,
            project: Option<Project>,
            title: &str,
            description: Option<String>,
            state: IssueState,
            source: &str,
            assignee: Option<User>,
        ) -> TodoIssue {
            TodoIssue {
                uuid: uuid::Uuid::new_v4(),
                source_id: source_id,
                project: project,
                title: String::from(title),
                description: description,
                state: state,
                source: String::from(source),
                assignee: assignee,
            }
        }

        pub fn get_source(&self) -> &String {
            &self.source
        }
    }

    impl fmt::Display for TodoIssue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                writeln!(f, "========================")?;
                writeln!(f, "Title: {}", self.title)?;
                if self.description.is_some() {
                    writeln!(f, "Description: {}", self.description.as_ref().unwrap_or(&String::from("No description available")))?;
                }
                writeln!(f, "Source: {}", self.source)?;
                writeln!(f, "========================")
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Project {
        source_id: i32,
        title: String,
        description: Option<String>,
        web_url: Option<String>,
        owner: Owner,
    }

    impl Project {
        pub fn new(
            source_id: i32,
            title: &str,
            description: Option<String>,
            web_url: Option<String>,
            owner: Owner,
        ) -> Project {
            Project {
                source_id: source_id,
                title: String::from(title),
                description: description,
                web_url: web_url,
                owner: owner,
            }
        }
    }

    impl fmt::Display for Project {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Project Title: {}", self.title)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
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
    #[derive(Debug, Serialize, Deserialize)]
    pub enum OwnerType {
        User,
    }

    /// The valid states an issue can be in
    #[derive(Debug, Serialize, Deserialize)]
    pub enum IssueState {
        Open,
        Closed,
    }

    #[derive(Debug, Serialize, Deserialize)]
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
