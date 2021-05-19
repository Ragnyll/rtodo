use reqwest::Result;
use reqwest::Client;
use reqwest::{ClientBuilder, header};
use serde_json::*;

use std::time::Duration;

struct GitlabApiConf {
    base_url: String,
    access_token: String,
    username: String,
    timeout: Duration,
}

impl GitlabApiConf {
    fn new(base_url: &str, access_token: &str, username: &str, timeout: Duration) -> GitlabApiConf {
        GitlabApiConf {
            base_url: String::from(base_url),
            access_token: String::from(access_token),
            username: String::from(username),
            timeout: timeout,
        }
    }

    fn get_base_url(&self) -> &str {
        &self.base_url
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_timeout(&self) -> Duration {
        self.timeout
    }

    fn get_accesss_token(&self) -> &str {
        &self.access_token
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    const BASE_URL: &str = "https://gitlab.com/api/v4/";
    const USERNAME: &str = "Ragnyll";
    let timeout = Duration::new(5, 0);

    let gitlab_api_conf = GitlabApiConf::new(BASE_URL, "SECRET", USERNAME, timeout);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "PRIVATE-TOKEN",
        header::HeaderValue::from_static("SECRET"),
    );
    let client = ClientBuilder::new()
        .timeout(gitlab_api_conf.get_timeout())
        .default_headers(headers)
        .build()?;

    println!("{}", determine_user_id(gitlab_api_conf, client).await);

    Ok(())

    // decrypt access token: default to decrypting from .password_store dir but allow override
    // figure our the user id and cache it
    // /users?username=<username
    // >>> response = json.loads(requests.get('https://gitlab.com/api/v4/users?username=ragnyll', headers={'Authorization': 'access_token ...........'}).text)
    // >>> user = json.loads(requests.get('https://gitlab.com/api/v4/users/2436873', headers={'Authorization': 'access_token .........'}).text)
    //
    // get projects belonging to user
    //
    // >>> projects = json.loads(requests.get('https://gitlab.com/api/v4/users/2436873/projects', headers={'Authorization': 'access_token ...........'}).text)
    //
    // Get all issues assigned to and logged by the user
    // >>> response = json.loads(requests.get('https://gitlab.com/api/v4/issues?assignee_id=2436873', headers={'PRIVATE-TOKEN': '.........'}).text)
    //
    // >>> response = json.loads(requests.get('https://gitlab.com/api/v4/issues?author_id=2436873', headers={'PRIVATE-TOKEN': '.............'}).text)
    //
    // Merge on project title to lightweight object
    //
    // merge to ~/todo.md
}

async fn determine_user_id(gitlab_api_conf: GitlabApiConf, client: Client) -> String {
    let user_url = format!(
        "{}/users?username={}",
        gitlab_api_conf.get_base_url(),
        gitlab_api_conf.get_username()
    );

    match client.get(&user_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let bytes = response.bytes().await.expect("Unable to deserialize response from user_url to bytes");
                let value: Value = serde_json::from_str(std::str::from_utf8(&bytes).expect("Invalid utf8 sequence")).expect("unable to deserialze response to json value");
                // i should do a length check on value first to make sure that there arent multiple users
                // and maybe a check to see that id exists
                println!("{}", value.get(0).unwrap()["id"]);
                return String::from("oh hi");
            } else {
                // TODO: dont panic just exit with an error code
                panic!("User_url request was not succesful with code : {}", response.status());
            }
        }
        _ => {
            return String::from("Unable to complete request to user url");
        }
    }
}
