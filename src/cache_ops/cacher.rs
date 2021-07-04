use std::fmt;
use std::fs::File;
use crate::todo_issues::TodoIssue;

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
        return Ok(vec!());
    }

    let todos: Vec<TodoIssue> = serde_json::from_str(&std::fs::read_to_string(cache_path)?)?;
    Ok(match filter {
        Some(f) => f(todos),
        None => todos,
    })
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
    ).expect("Unable to read local issues into memory"))
}

pub fn read_all_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    let filtered_todos = read_into_mem(
        cache_path,
        // TODO: This is stupid. fix this
        Some(|todos: Vec<TodoIssue>| -> Vec<TodoIssue> {
            todos
        }),
    )
    .expect("Unable to read all issues into memory");
    Ok(filtered_todos)
}

pub fn insert_into_cache(cache_path: &str, new_issues: TodoIssue) -> Result<(), CacheUpdateError> {
    let mut todo_issues: Vec<TodoIssue> = read_all_issues_to_mem(cache_path)?;
    Ok(todo_issues.push(new_issues))
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
