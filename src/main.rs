#[macro_use]
extern crate serde_derive;
extern crate exitcode;

use std::time::Duration;

use reqwest::Result;

mod gitlab_api_objects;
use crate::gitlab_api_objects::GitlabApiConf;
use crate::gitlab_api_objects::GitlabApiClient;

#[tokio::main]
async fn main() -> Result<()> {
    const BASE_URL: &str = "https://gitlab.com/api/v4/";
    const USERNAME: &str = "Ragnyll";
    let timeout = Duration::new(5, 0);

    let gitlab_api_conf = GitlabApiConf::new(BASE_URL, "SECRET", USERNAME, timeout);
    let gitlab_api_client =
        GitlabApiClient::new(gitlab_api_conf).expect("Unable to create GitlabApiClient");

    let user_id = gitlab_api_client.determine_user_id().await;
    println!("{:?}", user_id);
    let user_projects = gitlab_api_client
        .get_projects_belonging_to_user(&user_id)
        .await;
    println!(" projects {:?}", user_projects.get(0));
    // let assigned_issues = get_all_issues_assigned_to_user(&gitlab_api_conf, &client, &user_id).await;
    // let reported_issues = get_all_issues_reported_by_user(&gitlab_api_conf, &client, &user_id).await;

    Ok(())
}
