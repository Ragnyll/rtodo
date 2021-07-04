use std::error::Error;
use std::io::Read;
use std::fmt;
use std::fs::File;

const DEFAULT_CONF_PATH: &str = ".config/rtodo/conf.json";

/// Master conf containing all the individual conf objects and utilities for building them
#[derive(Clone, Debug, Deserialize)]
pub struct Conf {
    // TODO: change offline mode to an enum with 3 modes "online","conservative","offline"
    offline_mode: bool,
    gitlab_api_conf: Option<super::gitlab_api_conf::GitlabApiConf>,
}

impl Conf {
    pub fn new(conf_path: &str) -> Result<Conf, ConfCreationError> {
        let mut conf_data = String::new();
        File::open(conf_path)?.read_to_string(&mut conf_data)?;

        let conf_data: Conf = serde_json::from_str(&conf_data)?;

        Ok(conf_data)
    }

    pub fn get_gitlab_api_conf(&self) -> &Option<super::GitlabApiConf> {
        return &self.gitlab_api_conf;
    }
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

impl From<std::io::Error> for ConfCreationError {
    fn from(err: std::io::Error) -> Self {
        ConfCreationError::new(&err.to_string())
    }
}

impl From<serde_json::Error> for ConfCreationError {
    fn from(err: serde_json::Error) -> Self {
        ConfCreationError::new(&err.to_string())
    }
}

// TODO: impl deserialize for duration
