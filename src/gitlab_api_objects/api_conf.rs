use std::time::Duration;

pub struct GitlabApiConf {
    base_url: String,
    access_token: String,
    username: String,
    timeout: Duration,
}

impl GitlabApiConf {
    pub fn new(base_url: &str, access_token: &str, username: &str, timeout: Duration) -> GitlabApiConf {
        GitlabApiConf {
            base_url: String::from(base_url),
            access_token: String::from(access_token),
            username: String::from(username),
            timeout: timeout,
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    pub fn get_accesss_token(&self) -> &str {
        &self.access_token
    }
}
