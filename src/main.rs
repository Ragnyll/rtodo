#[macro_use]
extern crate serde_derive;
extern crate exitcode;
extern crate bytes;

use reqwest::Result;

mod models;
use crate::models::gitlab_api_objects::GitlabApiClient;
use crate::models::todo_issues;
use crate::models::todo_issues::todo_issue::todo_issues::Convertable;
use crate::models::todo_issues::todo_issue::todo_issues::TodoIssue;
use crate::converters::GitlabIssueContainer;

mod converters;

mod conf;
use conf::conf::Conf;

mod cache_ops;
use cache_ops::cacher::{read_all_issues_to_mem, read_local_issues_to_mem, write_to_cache_file};

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Conf::new(None).expect("Unable to construct conf object");
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

    let mut todos: Vec<TodoIssue> = vec!();
    for issue in issues_assigned_to_user {
        // TODO: DEFECT user projects does not account for projects not owned by user_id
        // TODO: clone is unnessecarily expensive. just figure out the lifetime
        let gitlab_issue_container = GitlabIssueContainer::new(issue, user_projects.clone());
        let todo = gitlab_issue_container.convert_to_todo_issue().expect(&String::from(format!("Unable to convert issue {:?} into a todo", gitlab_issue_container)));
        todos.push(todo);
    }

    // write_to_cache_file("/home/ragnyll/.cache/rust-todo.json", todos).expect("Couldnt write to cache file");
    let all_issues = read_all_issues_to_mem("/home/ragnyll/.cache/rust-todo.json").expect("couldnt read into memory");
    println!("{:?}", all_issues);

    Ok(())
}
