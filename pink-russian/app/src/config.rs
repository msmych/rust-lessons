use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub db: Arc<DbConfig>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DbConfig {
    pub address: String,
    pub username: String,
    pub password: String,
    pub ns: String,
    pub db: String,
}
