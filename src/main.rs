#[macro_use]
extern crate serde_derive;
extern crate exitcode;

use reqwest::Result;

mod gitlab_api_objects;
use crate::gitlab_api_objects::GitlabApiClient;

mod conf;
use conf::conf::Conf;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Conf::new(None).expect("Unable to construst conf object");
    let gitlab_api_conf = conf
        .get_gitlab_api_conf()
        .clone()
        .expect("conf does not have a gitlab_api_conf");
    let gitlab_api_client =
        GitlabApiClient::new(gitlab_api_conf).expect("Unable to create GitlabApiClient");

    let user_id = gitlab_api_client.determine_user_id().await;
    println!("{:?}", user_id);
    let user_projects = gitlab_api_client
        .get_projects_belonging_to_user(&user_id)
        .await;
    println!("projects {:?}", user_projects.get(0));
    let user = gitlab_api_client
        .get_gitlab_user(&user_id)
        .await;
    println!("user {:?}", user);
    let issues_assigned_to_user = gitlab_api_client.get_issues_assigned_to_user(&user_id).await;
    println!("assigned_issues {:?}", issues_assigned_to_user);
    // let reported_issues = gitlab_api_client.get_issues_reported_by_user(&user_id).await;
    // println!("reported_issues {:?}", reported_issues);

    Ok(())
}
