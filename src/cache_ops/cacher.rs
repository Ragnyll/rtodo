use std::fmt;
use crate::todo_issues::TodoIssue;

/// Writes the Vec<TodoIssue> to the given file at cache_path.
/// If it does not exist at that path a new file will be created at that location
pub fn write_to_cache_file(cache_path: &str, todo_issues: Vec<TodoIssue>) -> Result<(), CacheWriteError> {
    Ok(())
}

fn read_into_mem(cache_path: &str, filter: Option<&dyn Fn(Vec<TodoIssue>) -> Result<Vec<TodoIssue>, CacheReadError>>) -> Result<Vec<TodoIssue>, CacheReadError> {
    println!("boss");
    Ok(vec!())
}

pub fn read_local_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    Ok(vec!())
}

pub fn read_all_issues_to_mem(cache_path: &str) -> Result<Vec<TodoIssue>, CacheReadError> {
    Ok(vec!())
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
