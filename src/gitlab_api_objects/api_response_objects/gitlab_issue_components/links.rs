#[derive(Deserialize, Debug)]
#[serde(rename = "_links")]
pub struct Links {
    #[serde(rename = "self")]
    links_self: Option<String>,
    notes: Option<String>,
    award_emoji: Option<String>,
    project: Option<String>,
}
