#[derive(Deserialize, Debug)]
pub struct Group {
    group_id: i32,
    group_name: String,
    group_full_path: String,
    group_access_level: i32,
}
