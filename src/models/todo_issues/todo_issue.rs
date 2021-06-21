mod todo_issues {
    /// A struct representing an issue todo
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

    pub struct Project {
        id: i32,
        title: String,
        description: Option<String>,
        web_url: String,
        owner: Owner,
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

    // TODO: find other types
    pub enum OwnerType {
        User,
    }


    /// The valid states an issue can be in
    pub enum IssueState {
        Open,
        InProgress,
        Blocked,
        Closed,
    }


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
        fn convertToTodoIssue(&self) -> TodoIssue;
    }
}
