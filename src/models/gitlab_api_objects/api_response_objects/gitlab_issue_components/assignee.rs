#[derive(Deserialize, Debug)]
pub struct Assignee {
    id: i32,
    name: Option<String>,
    username: Option<String>,
    state: Option<String>,
    avatar_url: Option<String>,
    web_url: Option<String>,
}

impl Assignee {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }
}
