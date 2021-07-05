use reqwest::{Client, ClientBuilder};
use super::api_response_objects::{GitlabProject, GitlabUser, GitlabIssue};
use reqwest::header;
use serde_json::Value;
use std::error::Error;
use std::fmt;

use crate::conf::gitlab_api_conf::GitlabApiConf;

const DEFAULT_TIMEOUT_SECONDS: u64 = 5;

pub struct GitlabApiClient {
    client: Client,
    conf: GitlabApiConf,
}

impl GitlabApiClient {
    pub fn new(gitlab_conf: GitlabApiConf) -> Result<GitlabApiClient, ClientCreationError> {
        let mut default_headers = header::HeaderMap::new();
        let token_header_value = header::HeaderValue::from_str(gitlab_conf.get_accesss_token())?;
        default_headers.insert("PRIVATE-TOKEN", token_header_value);

        Ok(GitlabApiClient {
            client: ClientBuilder::new()
                .timeout(match gitlab_conf.get_timeout() {
                    Some(t) => t,
                    None => std::time::Duration::new(DEFAULT_TIMEOUT_SECONDS, 0),
                })
                .default_headers(default_headers)
                .build()?,
            conf: gitlab_conf,
        })
    }

    /// Pull only the user from gitlab respons
    pub async fn determine_user_id(&self) -> Result<String, ClientResponseError> {
        let user_url = format!(
            "{}/users?username={}",
            self.conf.get_base_url(),
            self.conf.get_username()
        );

        let response = self.client.get(&user_url).send().await?;
        if response.status().is_success() {
            let bytes: bytes::Bytes = response.bytes().await?;
            let value: Value = serde_json::from_str(std::str::from_utf8(&bytes)?)?;
            // This is brittle but i dont really care. I cant think of a real case where len > 1
            return Ok(String::from(format!("{}", value.get(0).unwrap()["id"])));
        } else {
            return Err(ClientResponseError::new(
                "Unsuccesful Response {} from url {}",
            ));
        }
    }

    /// Returns a list of project id belonging to user_id
    pub async fn get_projects_belonging_to_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<GitlabProject>, ClientResponseError> {
        let url = format!("{}/users/{}/projects", self.conf.get_base_url(), user_id);

        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            return Ok(response.json::<Vec<GitlabProject>>().await?);
        }
        Err(ClientResponseError::new(&format!(
            "Unsuccesful Response {} from url {}",
            response.status(),
            url
        )))
    }

    /// Returns a list of project id belonging to user_id
    #[allow(dead_code)]
    pub async fn get_gitlab_user(&self, user_id: &str) -> Result<GitlabUser, ClientResponseError> {
        let url = format!("{}/users/{}", self.conf.get_base_url(), user_id);

        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            return Ok(response.json::<GitlabUser>().await?);
        }
        Err(ClientResponseError::new(&format!(
            "Unsuccesful Response {} from url {}",
            response.status(),
            url
        )))
    }

    pub async fn get_issues_assigned_to_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<GitlabIssue>, ClientResponseError> {
        let url = format!(
            "{}/issues?assignee_id={}",
            self.conf.get_base_url(),
            user_id
        );

        let response = self.client.get(&url).send().await?;
        if response.status().is_success() {
            return Ok(response.json::<Vec<GitlabIssue>>().await?);
        }
        Err(ClientResponseError::new(&format!(
            "Unsuccesful Response {} from url {}",
            response.status(),
            url
        )))
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
        write!(f, "{}", self.details)
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

#[derive(Debug)]
pub struct ClientResponseError {
    details: String,
}

impl ClientResponseError {
    fn new(msg: &str) -> ClientResponseError {
        ClientResponseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ClientResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<reqwest::Error> for ClientResponseError {
    fn from(err: reqwest::Error) -> Self {
        ClientResponseError::new(&err.to_string())
    }
}

impl From<std::str::Utf8Error> for ClientResponseError {
    fn from(err: std::str::Utf8Error) -> Self {
        ClientResponseError::new(&err.to_string())
    }
}

impl From<serde_json::Error> for ClientResponseError {
    fn from(err: serde_json::Error) -> Self {
        ClientResponseError::new(&err.to_string())
    }
}
