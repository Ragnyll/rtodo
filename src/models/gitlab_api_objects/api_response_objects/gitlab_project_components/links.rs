#[derive(Deserialize, Debug)]
pub struct Links {
    link_to_self: Option<String>, // this is self on the api i dont know how this translates to self
    issues: Option<String>,
    merge_requests: Option<String>,
    repo_branches: Option<String>,
    labels: Option<String>,
    events: Option<String>,
    members: Option<String>,
}
