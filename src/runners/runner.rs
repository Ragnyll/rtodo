use std::fmt;

use crate::cli::cli_parser::CommandConf;
use crate::cache_ops::cacher::{read_all_issues_to_mem, read_local_issues_to_mem, write_to_cache_file};
use crate::models::gitlab_api_objects::GitlabApiClient;
use crate::converters::GitlabIssueContainer;
use crate::models::todo_issue::todo_issues::TodoIssue;
use crate::conf::conf::Conf;
use crate::models::todo_issue::todo_issues::Convertable;
use crate::cache_ops::cacher::CacheWriteError;

const DEFAULT_REFRESH_TIME_MINUTES: u32 = 300;

/// Runs with the given configuration from the cli
pub async fn run_with_configuration(cli_conf: CommandConf) -> Result<(), RunError> {
    let conf = Conf::new(&cli_conf.conf_path).expect("Unable to create configuration");

    if cli_conf.new_todo.is_some() {
        println!("Creating new todo")
    } else if cli_conf.delete_todo.is_some() {
        println!("Deleting todo")
    } else if cli_conf.no_ui {
        print_all_todos(cli_conf, conf).await?;
    } else {
        eprintln!("TUI interface not supported yet");
    }

    Ok(())
}

/// TODO: do the time diff check
fn should_update_cache(conf: &CommandConf, refresh_time_min: u32) -> bool {
    true
}

async fn update_cache_from_remote_issues(
    conf: Conf,
    cli_conf: &CommandConf,
) -> Result<(), CacheWriteError> {
    let mut all_issues: Vec<TodoIssue> = read_local_issues_to_mem(&cli_conf.cache_path)
        .expect("Unable to read local issues into memory");

    all_issues.append(&mut update_issues_from_gitlab(conf, &cli_conf).await);

    write_to_cache_file(&cli_conf.cache_path, all_issues)
}

async fn update_issues_from_gitlab(conf: Conf, cli_conf: &CommandConf) -> Vec<TodoIssue> {
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

    let mut todos = read_local_issues_to_mem(&cli_conf.cache_path).expect(&format!(
        "Could not read cache file {}",
        cli_conf.cache_path
    ));
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

async fn print_all_todos(cli_conf: CommandConf, conf: Conf) -> Result<(), CacheWriteError> {
    if should_update_cache(&cli_conf, DEFAULT_REFRESH_TIME_MINUTES) {
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

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
