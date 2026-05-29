use crate::{auth::AuthService, config::Config};

#[derive(Debug)]
pub struct AppState {
    pub config: Config,
    pub auth: AuthService,
    pub started_at: std::time::SystemTime,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        Ok(Self { auth: AuthService::new(&config.system_root), config, started_at: std::time::SystemTime::now() })
    }

    pub async fn shutdown(&self) {
        tracing::info!("shutdown sequence complete");
    }
}
