#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub LOCALIP: String,
    pub SERVERIP: String,
}
