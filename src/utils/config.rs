#[derive(Deserialize)]
struct Config {
    server: Option<Server>,
    paths: Option<Paths>,
}

#[derive(Deserialize)]
struct Server {
    ip: Option<String>,
    port: Option<u64>,
}

#[derive(Deserialize)]
struct Paths {
    template_folder: Option<String>,
    admin_theme: Option<String>,
    content_themes: Option<String>,
    active_theme: Option<String>,
}