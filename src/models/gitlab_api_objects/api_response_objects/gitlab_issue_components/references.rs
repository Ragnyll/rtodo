#[derive(Deserialize, Debug)]
pub struct References {
    short: Option<String>,
    relative: Option<String>,
    full: Option<String>,
}
