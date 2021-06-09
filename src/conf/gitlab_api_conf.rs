use std::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub struct GitlabApiConf {
    base_url: String,
    access_token: String,
    username: String,
    timeout: Option<Duration>,
}

impl GitlabApiConf {
    #[allow(dead_code)]
    pub fn new(
        base_url: &str,
        access_token: &str,
        username: &str,
        timeout: Duration,
    ) -> GitlabApiConf {
        GitlabApiConf {
            base_url: String::from(base_url),
            access_token: String::from(access_token),
            username: String::from(username),
            timeout: Some(timeout),
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_timeout(&self) -> Option<Duration> {
        self.timeout
    }

    pub fn get_accesss_token(&self) -> &str {
        &self.access_token
    }
}
