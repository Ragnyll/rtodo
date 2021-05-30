#[derive(Deserialize, Debug)]
pub struct Namespace {
  id: i32,
  name: String,
  path: String,
  kind: String,
  full_path: String,
}
