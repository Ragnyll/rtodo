use super::gitlab_project_components::Namespace;

#[derive(Deserialize, Debug)]
pub struct GitlabProject {
    id: i32,
    description: Option<String>,
    name: Option<String>,
    name_with_namespace: Option<String>,
    path: Option<String>,
    path_with_namespace: Option<String>,
    created_at: Option<String>,
    default_branch: Option<String>,
    tag_list: Vec<String>,
    ssh_url_to_repo: Option<String>,
    http_url_to_repo: Option<String>,
    web_url: Option<String>,
    readme_url: Option<String>,
    avatar_url: Option<String>,
    forks_count: i32,
    star_count: i32,
    last_activity_at: Option<String>,
    namespace: Namespace,
}

impl GitlabProject {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    pub fn get_web_url(&self) -> &Option<String> {
        &self.web_url
    }

    pub fn get_namespace(&self) -> &Namespace {
        &self.namespace
    }
}
