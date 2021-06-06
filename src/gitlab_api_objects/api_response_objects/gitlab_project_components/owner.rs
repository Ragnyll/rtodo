#[derive(Deserialize, Debug)]
pub struct Owner {
    id: i32,
    name: Option<String>,
    created_at: Option<String>,
}
