extern crate dirs;

use std::error::Error;
use std::fmt;
use std::time::Duration;
use std::path::PathBuf;

/// Master conf containing all the individual conf objects and utilities for building them
pub struct Conf {
    gitlab_api_conf: Option<super::gitlab_api_conf::GitlabApiConf>,
}

impl Conf {
    pub fn new() -> Result<Conf, ConfCreationError> {
        let home_dir = find_home_dir()?;
        let BASE_URL = "https://gitlab.com/api/v4/";
        let USERNAME = "Ragnyll";
        let TOKEN = "SECRET";
        let timeout = Duration::new(5, 0);

        Ok(Conf {
            gitlab_api_conf: Some(super::GitlabApiConf::new(
                BASE_URL, TOKEN, USERNAME, timeout,
            )),
        })
    }

    pub fn get_gitlab_api_conf(&self) -> &Option<super::GitlabApiConf> {
        return &self.gitlab_api_conf;
    }
}

/// Finds the home directory or errors in the process
fn find_home_dir() -> Result<String, ConfCreationError> {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(ConfCreationError::new("Unable to find home_dir"));
        }
    };

    return match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => Err(ConfCreationError::new("Unable to deterimine home_dir path")),
    };
}

#[derive(Debug)]
pub struct ConfCreationError {
    details: String,
}

impl ConfCreationError {
    fn new(msg: &str) -> ConfCreationError {
        ConfCreationError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConfCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConfCreationError {
    fn description(&self) -> &str {
        &self.details
    }
}
