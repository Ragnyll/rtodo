#[derive(Deserialize, Debug)]
pub struct GitlabUser {
    id: i32,
    name: Option<String>,
    username: Option<String>,
    state: Option<String>,
    avatar_url: Option<String>,
    web_url: Option<String>,
    created_at: Option<String>,
    bio: Option<String>,
    bio_html: Option<String>,
    location: Option<String>,
    public_email: Option<String>,
    skype: Option<String>,
    linkedin: Option<String>,
    twitter: Option<String>,
    website_url: Option<String>,
    organization: Option<String>,
    job_title: Option<String>,
    bot: bool,
    work_information: Option<String>,
    followers: i32,
    following: i32,
}

impl GitlabUser {
    pub fn get_id(&self) -> i32 {
        self.id
    }


    pub fn get_username(&self) -> &Option<String> {
        &self.username
    }

    pub fn get_email(&self) -> &Option<String> {
        &self.public_email
    }
}