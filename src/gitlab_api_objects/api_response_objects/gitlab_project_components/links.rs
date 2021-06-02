#[derive(Deserialize, Debug)]
pub struct Links {
  link_to_self: String, // this is self on the api i dont know how this translates to self
  issues: String,
  merge_requests: String,
  repo_branches: String,
  labels: String,
  events: String,
  members: String,
}
