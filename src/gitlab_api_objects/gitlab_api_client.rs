use std::time::Duration;
use reqwest::{ Client, ClientBuilder };
use serde_json::Value;

pub struct GitlabApiClient {
    client: reqwest::Client,
}

impl GitlabApiClient {
    pub fn new() -> GitlabApiClient {
        GitlabApiClient {
            client: ClientBuilder::new()
            .timeout(Duration::new(5,0))
            // .default_headers(headers)
            .build().unwrap()
        }
    }

    /// Pull only the user from gitlab respons
    async fn determine_user_id(gitlab_api_conf: &super::api_conf::GitlabApiConf, client: &Client) -> String {
        let user_url = format!(
            "{}/users?username={}",
            gitlab_api_conf.get_base_url(),
            gitlab_api_conf.get_username()
        );

        let response = client
            .get(&user_url)
            .send()
            .await
            .expect("Did not receive a response from user_url");
        if response.status().is_success() {
            let bytes = response
                .bytes()
                .await
                .expect("Unable to deserialize response from user_url to bytes");
            let value: Value =
                serde_json::from_str(std::str::from_utf8(&bytes).expect("Invalid utf8 sequence"))
                    .expect("unable to deserialze response to json value");
            // This is brittle but i dont really care. I cant think of a real case where len > 1
            return String::from(format!("{}", value.get(0).unwrap()["id"]));
        } else {
            eprintln!(
                "Unsuccesful Response {} from url {}",
                response.status(),
                user_url
            );
            std::process::exit(exitcode::DATAERR);
        }
    }

}
