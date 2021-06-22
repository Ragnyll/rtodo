#[derive(Deserialize, Debug, Clone)]
pub struct Namespace {
    id: i32,
    name: Option<String>,
    path: Option<String>,
    kind: Option<String>,
    full_path: Option<String>,
    parent_id: Option<String>,
    avatar_url: Option<String>,
    web_url: Option<String>,
}

impl Namespace {
    pub fn get_id(&self) -> i32 {
        self.id
    }
}
