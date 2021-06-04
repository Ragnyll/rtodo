#[macro_use]
extern crate serde_derive;
extern crate exitcode;

use std::time::Duration;

use reqwest::Result;
use reqwest::Client;

mod gitlab_api_objects;
use crate::gitlab_api_objects::GitlabProject;
use crate::gitlab_api_objects::GitlabApiConf;
use crate::gitlab_api_objects::GitlabApiClient;

#[tokio::main]
async fn main() -> Result<()> {
    const BASE_URL: &str = "https://gitlab.com/api/v4/";
    const USERNAME: &str = "Ragnyll";
    let timeout = Duration::new(5, 0);

    let gitlab_api_conf = GitlabApiConf::new(BASE_URL, "SECRET", USERNAME, timeout);
    let gitlab_api_client = GitlabApiClient::new(gitlab_api_conf).expect("Unable to create GitlabApiClient");

    let user_id = gitlab_api_client.determine_user_id().await;
    println!("{:?}", user_id);
    // let user_projects = get_projects_belonging_to_user(&gitlab_api_conf, &client, &user_id).await;
    // let assigned_issues = get_all_issues_assigned_to_user(&gitlab_api_conf, &client, &user_id).await;
    // let reported_issues = get_all_issues_reported_by_user(&gitlab_api_conf, &client, &user_id).await;


    Ok(())

}

async fn get_all_issues_assigned_to_user(gitlab_api_conf: &GitlabApiConf, client: &Client, user_id: &str) -> String {
    return String::from("heck");
}

async fn get_all_issues_reported_by_user(gitlab_api_conf: &GitlabApiConf, client: &Client, user_id: &str) -> String {
    return String::from("heck");
}


/// Returns a list of project id belonging to user_id
async fn get_projects_belonging_to_user(
    gitlab_api_conf: &GitlabApiConf,
    client: &Client,
    user_id: &str,
) -> Vec<u64> {
    let project_url = format!(
        "{}/users/{}/projects",
        gitlab_api_conf.get_base_url(),
        user_id,
    );

    let response = client
        .get(&project_url)
        .send()
        .await
        .expect("Did not receive a response from project_url");
    if response.status().is_success() {
        // doesnt work quite yet
        let projects = response.json::<GitlabProject>().await.expect("Could not deserialze json into projects");
        println!("{:?}", projects);

        let bs: Vec<u64> = vec!();
        return bs;
    } else {
        eprintln!(
            "Unsuccesful Response {} from url {}",
            response.status(),
            user_id
        );
        std::process::exit(exitcode::DATAERR);
    }
}
