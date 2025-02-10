#[derive(Debug, Clone)]
pub struct App {
    pub title: String,
    pub version: String,
    pub host: String,
    pub port: u32,
    pub workers: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: std::env::var("SERVICE_TITLE").unwrap_or_default(),
            version: std::env::var("SERVICE_VERSION").unwrap_or_default(),
            host: std::env::var("SERVICE_HOST").unwrap_or_default(),
            port: std::env::var("SERVICE_PORT")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_default(),
            workers: std::env::var("SERVICE_WORKERS")
                .unwrap()
                .parse::<usize>()
                .unwrap_or_default(),
        }
    }

    pub fn bind(&self) -> (String, u16) {
        (self.clone().host, self.clone().port as u16)
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "rust-service".into(),
            version: "0.1.0".into(),
            host: "localhost".into(),
            port: 6969,
            workers: 1_usize,
        }
    }
}
