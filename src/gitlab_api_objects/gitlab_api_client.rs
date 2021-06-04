use reqwest::{Client, ClientBuilder};
use reqwest::header;
use serde_json::Value;
use std::error::Error;
use std::fmt;

pub struct GitlabApiClient {
    client: Client,
    conf: super::GitlabApiConf,
}

impl GitlabApiClient {
    pub fn new(gitlab_conf: super::GitlabApiConf) -> Result<GitlabApiClient, ClientCreationError> {
        let mut default_headers = header::HeaderMap::new();
        let token_header_value = header::HeaderValue::from_str(gitlab_conf.get_accesss_token())?;
        default_headers.insert("PRIVATE-TOKEN", token_header_value);

        Ok(GitlabApiClient {
            client: ClientBuilder::new()
                .timeout(gitlab_conf.get_timeout())
                .default_headers(default_headers)
                .build()?,
            conf: gitlab_conf,
        })
    }

    /// Pull only the user from gitlab respons
    pub async fn determine_user_id(&self) -> String {
        let user_url = format!(
            "{}/users?username={}",
            self.conf.get_base_url(),
            self.conf.get_username()
        );

        let response = self
            .client
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

#[derive(Debug)]
pub struct ClientCreationError {
    details: String,
}

impl ClientCreationError {
    fn new(msg: &str) -> ClientCreationError {
        ClientCreationError {
            details: msg.to_string(),
        }
    }

}

impl fmt::Display for ClientCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ClientCreationError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<header::InvalidHeaderValue> for ClientCreationError {
fn from(err: header::InvalidHeaderValue) -> Self {
        ClientCreationError::new(&err.to_string())
    }
}

impl From<reqwest::Error> for ClientCreationError {
    fn from(err: reqwest::Error) -> Self {
        ClientCreationError::new(&err.to_string())
    }
}
