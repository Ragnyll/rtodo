use std::fmt;
use std::fs::File;
use crate::todo_issues::{TodoIssue, IssueState};

/// Writes the Vec<TodoIssue> to the given file at cache_path.
/// If it does not exist at that path a new file will be created at that location
pub fn write_to_cache_file(
    cache_path: &str,
    todo_issues: Vec<TodoIssue>,
) -> Result<(), CacheWriteError> {
    if !std::path::Path::new(cache_path).exists() {
        File::create(cache_path)?;
    }

    Ok(::serde_json::to_writer(
        &File::create(cache_path)?,
        &todo_issues,
    )?)
}

fn read_into_mem<F>(cache_path: &str, filter: Option<F>) -> Result<Vec<TodoIssue>, CacheReadError>
where
    F: Fn(Vec<TodoIssue>) -> Vec<TodoIssue>,
{
    if !std::path::Path::new(cache_path).exists() {
        File::create(cache_path)?;
        return Ok(vec![]);
    }

    let todos: Vec<TodoIssue> = serde_json::from_str(&std::fs::read_to_string(cache_path)?)?;
    Ok(match filter {
        Some(f) => f(todos),
        None => todos,
    })
}

pub fn read_issue_into_memory_by_uuid(
    cache_path: &str,
    uuid: uuid::Uuid,
) -> Result<Vec<TodoIssue>, CacheReadError> {
    Ok(read_into_mem(
        cache_path,
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
                .into_iter()
                .filter(|t| {
                    *t.get_uuid() == uuid
                        && match t.get_state() {
                            IssueState::Closed => false,
                            _ => true,
                        }
                })
                .collect::<Vec<TodoIssue>>()
        }),
    )
    .expect("Unable to read local issues into memory"))
}

pub fn read_all_unclosed_issues_into_mem_excluding_uuid(
    cache_path: &str,
    uuid: uuid::Uuid,
) -> Result<Vec<TodoIssue>, CacheReadError> {
    Ok(read_into_mem(
        cache_path,
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
                .into_iter()
                .filter(|t| {
                    *t.get_uuid() != uuid
                        && match t.get_state() {
                            IssueState::Closed => false,
                            _ => true,
                        }
                })
                .collect::<Vec<TodoIssue>>()
        }),
    )
    .expect("Unable to read local issues into memory"))
}

pub fn read_local_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    Ok(read_into_mem(
        cache_path,
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
                .into_iter()
                .filter(|t| t.get_source() == "LOCAL")
                .collect::<Vec<TodoIssue>>()
        }),
    )
    .expect("Unable to read local issues into memory"))
}

pub fn read_all_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    let filtered_todos = read_into_mem(
        cache_path,
        // TODO: This is stupid. fix this
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> { todos }),
    )
    .expect("Unable to read all issues into memory");
    Ok(filtered_todos)
}

pub fn read_all_unclosed_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    let filtered_todos = read_into_mem(
        cache_path,
        // TODO: This is stupid. fix this
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
                .into_iter()
                .filter(|t| match t.get_state() {
                    IssueState::Closed => false,
                    _ => true,
                })
                .collect::<Vec<TodoIssue>>()
        }),
    )
    .expect("Unable to read all issues into memory");
    Ok(filtered_todos)
}

pub fn close_specific_todo(id: &str, cache_path: &str) -> Result<(), CacheWriteError> {
    let all_issues: Vec<TodoIssue> = read_all_unclosed_issues_into_mem_excluding_uuid(
        cache_path,
        uuid::Uuid::parse_str(id).unwrap(),
    )
    .unwrap();
    write_to_cache_file(cache_path, all_issues).expect("unable to remove issue from todos");
    Ok(())
}

pub fn read_all_unclosed_issue_of_source_type_to_mem(
    cache_path: &str,
    issue_source_type: &str,
) -> Result<Vec<TodoIssue>, CacheReadError> {
    let filtered_todos = read_into_mem(
        cache_path,
        // TODO: This is stupid. fix this
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
                .into_iter()
                .filter(|t| match t.get_state() {
                    IssueState::Closed => false,
                    _ => t.source.to_lowercase().contains(issue_source_type),
                })
                .collect::<Vec<TodoIssue>>()
        }),
    )
    .expect("Unable to read all unclosed gitlab issues into memory");
    Ok(filtered_todos)
}

#[derive(Debug)]
pub struct CacheWriteError {
    details: String,
}

impl CacheWriteError {
    #[allow(dead_code)]
    pub fn new(msg: &str) -> CacheWriteError {
        CacheWriteError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CacheWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<serde_json::Error> for CacheWriteError {
    fn from(err: serde_json::Error) -> Self {
        CacheWriteError::new(&err.to_string())
    }
}

impl From<std::io::Error> for CacheWriteError {
    fn from(err: std::io::Error) -> Self {
        CacheWriteError::new(&err.to_string())
    }
}

#[derive(Debug)]
pub struct CacheReadError {
    details: String,
}

impl CacheReadError {
    #[allow(dead_code)]
    pub fn new(msg: &str) -> CacheReadError {
        CacheReadError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CacheReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<std::io::Error> for CacheReadError {
    fn from(err: std::io::Error) -> Self {
        CacheReadError::new(&err.to_string())
    }
}

impl From<serde_json::Error> for CacheReadError {
    fn from(err: serde_json::Error) -> Self {
        CacheReadError::new(&err.to_string())
    }
}

#[derive(Debug)]
pub struct CacheUpdateError {
    details: String,
}

impl CacheUpdateError {
    #[allow(dead_code)]
    pub fn new(msg: &str) -> CacheUpdateError {
        CacheUpdateError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CacheUpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<CacheReadError> for CacheUpdateError {
    fn from(err: CacheReadError) -> Self {
        CacheUpdateError::new(&err.to_string())
    }
}
