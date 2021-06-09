#[derive(Deserialize, Debug)]
pub struct ClosedBy {
    id: i32,
    name: Option<String>,
    username: Option<String>,
    state: Option<String>,
    avatar_url: Option<String>,
    web_url: Option<String>
}
