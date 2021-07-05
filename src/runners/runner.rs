use std::fmt;
use std::fs::metadata;
use std::time::SystemTime;

use crate::cli::cli_parser::CommandConf;
use crate::cache_ops::cacher::{
    read_all_issues_to_mem, read_local_issues_to_mem, write_to_cache_file, CacheReadError,
    CacheWriteError, read_all_unclosed_issues_to_mem,
    read_all_unclosed_issues_into_mem_excluding_uuid,
};
use crate::models::gitlab_api_objects::GitlabApiClient;
use crate::converters::GitlabIssueContainer;
use crate::models::todo_issue::todo_issues::{TodoIssue, IssueState};
use crate::conf::conf::Conf;
use crate::models::todo_issue::todo_issues::Convertable;

const DEFAULT_CACHE_REFRESH_TIME: u16 = 7200; // Cache will refresh every 2 hours

/// Runs with the given configuration from the cli
pub async fn run_with_configuration(cli_conf: CommandConf) -> Result<(), RunError> {
    let conf = Conf::new(&cli_conf.conf_path).expect("Unable to create configuration");

    if cli_conf.new_todo.is_some() {
        create_new_local_todo(&cli_conf)?;
    } else if cli_conf.close_todo.is_some() {
        close_todo(&cli_conf)?;
    } else if cli_conf.no_ui {
        print_all_unclosed_todos(cli_conf, conf).await?;
    } else {
        print_all_unclosed_todos(cli_conf, conf).await?;
    }

    Ok(())
}

fn close_todo(cli_conf: &CommandConf) -> Result<(), RunError> {
    // close_todo.unwrap should not error becasue the only path it is called from forces it to be
    // some
    let todos = read_all_unclosed_issues_into_mem_excluding_uuid(
        &cli_conf.cache_path,
        cli_conf.close_todo.unwrap(),
    )
    .expect(&format!(
        "Could not read cache file {}",
        cli_conf.cache_path
    ));

    Ok(write_to_cache_file(&cli_conf.cache_path, todos)?)
}

fn create_new_local_todo(cli_conf: &CommandConf) -> Result<(), RunError> {
    // Create the new issue from the title and desc
    let new_issue = TodoIssue::new(
        None,
        None,
        &cli_conf.new_todo.as_ref().unwrap().title,
        // clone the ref
        Some(cli_conf.new_todo.as_ref().unwrap().description.clone()),
        IssueState::Open,
        &"LOCAL",
        None,
    );
    let mut all_todos = read_all_issues_to_mem(&cli_conf.cache_path)?;
    all_todos.push(new_issue);
    Ok(write_to_cache_file(&cli_conf.cache_path, all_todos)?)
}

fn should_update_cache(conf: &CommandConf) -> bool {
    if conf.force_no_refresh_cache {
        return false;
    }

    if conf.force_refresh_cache || !std::path::Path::new(&conf.cache_path).exists() {
        return true;
    }

    // NOTE: not monotonic, but should be accurate enough for a refresh every few hours
    let now = SystemTime::now();
    let cache_modified_time = metadata(&conf.cache_path)
        .expect(&format!(
            "Unable to read metadata on cache file {}",
            &conf.cache_path
        ))
        .modified()
        .expect(&format!(
            "Unable to read modified time on cache file {}",
            &conf.cache_path
        ));

    now.duration_since(cache_modified_time)
        .expect("Unable to determine whether to update cache")
        .as_secs()
        >= DEFAULT_CACHE_REFRESH_TIME.into()
}

async fn update_cache_from_remote_issues(
    conf: Conf,
    cli_conf: &CommandConf,
) -> Result<(), CacheWriteError> {
    let mut all_issues: Vec<TodoIssue> = read_local_issues_to_mem(&cli_conf.cache_path)
        .expect("Unable to read local issues into memory");

    all_issues.append(&mut update_issues_from_gitlab(conf).await);

    write_to_cache_file(&cli_conf.cache_path, all_issues)
}

async fn update_issues_from_gitlab(conf: Conf) -> Vec<TodoIssue> {
    let gitlab_api_conf = conf
        .get_gitlab_api_conf()
        .clone()
        .expect("conf does not have a gitlab_api_conf");
    let gitlab_api_client =
        GitlabApiClient::new(gitlab_api_conf).expect("Unable to create GitlabApiClient");

    let user_id = gitlab_api_client
        .determine_user_id()
        .await
        .expect("Unable to determine user_id");
    let user_projects = gitlab_api_client
        .get_projects_belonging_to_user(&user_id)
        .await
        .expect("Unable to find user_projects");
    let issues_assigned_to_user = gitlab_api_client
        .get_issues_assigned_to_user(&user_id)
        .await
        .expect("Unable to get issues_assigned_to_user");

    let mut todos: Vec<TodoIssue> = vec![];
    for issue in issues_assigned_to_user {
        // TODO: DEFECT user projects does not account for projects not owned by user_id
        // TODO: clone is unnessecarily expensive. just figure out the lifetime
        let gitlab_issue_container = GitlabIssueContainer::new(issue, user_projects.clone());
        let todo = gitlab_issue_container
            .convert_to_todo_issue()
            .expect(&String::from(format!(
                "Unable to convert issue {:?} into a todo",
                gitlab_issue_container
            )));
        todos.push(todo);
    }

    todos
}

async fn print_all_unclosed_todos(
    cli_conf: CommandConf,
    conf: Conf,
) -> Result<(), CacheWriteError> {
    if should_update_cache(&cli_conf) {
        update_cache_from_remote_issues(conf, &cli_conf).await?;
    }

    let todos = read_all_unclosed_issues_to_mem(&cli_conf.cache_path).expect(&format!(
        "Could not read cache file {}",
        cli_conf.cache_path
    ));

    for todo in todos {
        println!("{}", todo);
    }

    Ok(())
}

#[allow(dead_code)]
async fn print_all_todos(cli_conf: CommandConf, conf: Conf) -> Result<(), CacheWriteError> {
    if should_update_cache(&cli_conf) {
        update_cache_from_remote_issues(conf, &cli_conf).await?;
    }

    let todos = read_all_issues_to_mem(&cli_conf.cache_path).expect(&format!(
        "Could not read cache file {}",
        cli_conf.cache_path
    ));

    for todo in todos {
        println!("{}", todo);
    }

    Ok(())
}

#[derive(Debug)]
pub struct RunError {
    details: String,
}

impl RunError {
    fn new(msg: &str) -> RunError {
        RunError {
            details: msg.to_string(),
        }
    }
}

impl From<CacheWriteError> for RunError {
    fn from(err: CacheWriteError) -> Self {
        RunError::new(&err.to_string())
    }
}

impl From<CacheReadError> for RunError {
    fn from(err: CacheReadError) -> Self {
        RunError::new(&err.to_string())
    }
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
