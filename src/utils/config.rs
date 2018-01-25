#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Option<Server>,
    pub paths: Option<Paths>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: Option<String>,
    pub port: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Paths {
    pub template_folder: Option<String>,
    pub admin_theme: Option<String>,
    pub content_themes: Option<String>,
    pub active_theme: Option<String>,
}