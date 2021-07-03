use clap::{App, Arg};
use std::fmt;
use std::path::PathBuf;

const DEFAULT_CONFIG_PATH: &str = &".config/rtodo/conf.json";
const DEFAULT_CACHE_PATH: &str = &".cache/rust-todo.json";

#[derive(Debug)]
pub struct CommandConf {
    conf_path: String,
    cache_path: String,
    force_refresh_cache: bool,
    force_no_refresh_cache: bool,
    no_ui: bool,
    new_todo: Option<NewTodo>,
}

impl CommandConf {
    fn new(conf_path: &str, cache_path: &str, force_refresh_cache: bool, force_no_refresh_cache: bool, no_ui: bool, new_todo: Option<NewTodo>) -> Result<Self, CommandLineParseError> {
        if force_refresh_cache && force_no_refresh_cache {
            return Err(CommandLineParseError::new("cache cannot be refreshed in offline mode"));
        }

        Ok(CommandConf {
            conf_path: String::from(conf_path),
            cache_path: String::from(cache_path),
            force_refresh_cache: force_refresh_cache,
            force_no_refresh_cache: force_no_refresh_cache,
            no_ui: no_ui,
            new_todo: new_todo
        })
    }
}

#[derive(Debug)]
struct NewTodo {
    project: String,
    description: String,
}

impl NewTodo {
    fn new(project: &str, description: &str) -> Self {
        NewTodo {
            project: String::from(project),
            description: String::from(description)
        }
    }
}

/// Finds the home directory or errors in the process
fn find_home_dir() -> Result<String, CommandLineParseError> {
    let home_dir: PathBuf = match dirs::home_dir() {
        Some(p) => p,
        None => {
            return Err(CommandLineParseError::new("Unable to find home_dir"));
        }
    };

    return match home_dir.into_os_string().into_string() {
        Ok(s) => Ok(s),
        Err(_) => Err(CommandLineParseError::new("Unable to deterimine home_dir path")),
    };
}

pub fn parse_line() -> CommandConf {
    let matches = App::new("rtodo")
        .version("0.1")
        .author("Ragnyll <ragnyll@gallowzhumour.dev>")
        .about("manages todos and issues across disparate sources")
        .arg(
            Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("The path to a custom config file")
            .takes_value(true),
        )
        .arg(
            Arg::new("cache-file")
            .short('f')
            .long("cache-file")
            .value_name("FILE")
            .about("The path of the cache-file to read and write from")
            .takes_value(true),
        )
        .arg(
            Arg::new("refresh-cache")
            .short('r')
            .long("refresh-cache")
            .about("forces a refresh of the non-local issues in the cache")
            .takes_value(false),
        )
        .arg(
            Arg::new("offline")
            .short('o')
            .long("offline")
            .about("do not refresh non-local issues in the cache")
            .takes_value(false),
        )
        .arg(
            Arg::new("no-ui")
            .long("no-ui")
            .about("just dump the cache file to stdout")
            .takes_value(false),
        )
        .subcommand(
            App::new("new")
            .about("creates a new local issue")
            .arg(
                Arg::new("project")
                .required(true)
                .about("The project to create the todo for")
            )
            .arg(
                Arg::new("description")
                .required(true)
                .about("the description of the todo")
            ),
        )
        .get_matches();

    // TODO: Im sure theres a more efficient way of doing this default logic
    let home_dir = find_home_dir().expect("Unable to determine home dir");
    let absolute_default_conf_path = format!("{}/{}", home_dir, DEFAULT_CONFIG_PATH);
    let absolute_default_cache_path = format!("{}/{}", home_dir, DEFAULT_CACHE_PATH);

    CommandConf::new(
        match matches.value_of("config") {
            Some(c) => c,
            None => &absolute_default_conf_path,
        },
        match matches.value_of("cache-file") {
            Some(c) => c,
            None => &absolute_default_cache_path
        },
        match matches.value_of("refresh-cache") {
            Some(_) => true,
            None => false
        },
        match matches.value_of("offline") {
            Some(_) => true,
            None => false,
        },
        match matches.value_of("no-ui") {
            Some(_) => true,
            None => false,
        },
        match matches.subcommand() {
            Some(("new", new_matches)) => Some(NewTodo::new(
                    new_matches.value_of("project").unwrap(),
                    new_matches.value_of("description").unwrap())),
            _ => None
        }
    ).expect("Unable to parse the command line")
}



#[derive(Debug)]
pub struct CommandLineParseError {
    details: String,
}

impl CommandLineParseError {
    fn new(msg: &str) -> CommandLineParseError {
        CommandLineParseError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CommandLineParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}