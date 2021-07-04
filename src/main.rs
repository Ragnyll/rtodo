#[macro_use]
extern crate serde_derive;
extern crate exitcode;
extern crate bytes;

use reqwest::Result;

mod models;
use crate::models::todo_issues;

mod converters;

mod conf;

mod cache_ops;

mod runners;
use runners::runner;

mod cli;
use cli::cli_parser::parse_line;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(runner::run_with_configuration(parse_line())
        .await
        .expect("Unable to run with the supplied configuration"))
}
