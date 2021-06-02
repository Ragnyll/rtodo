#[derive(Deserialize, Debug)]
pub struct Statistics {
  commit_count: i32,
  storage_size: i32,
  repository_size: i32,
  wiki_size : i32,
  lfs_objects_size: i32,
  job_artifacts_size: i32,
  packages_size: i32,
  snippets_size: i32,
}
