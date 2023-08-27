#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub local_ip: String,
    pub server_ip: String,
}
