use std::sync::Arc;

use app::config::AppConfig;
use app::db::setup_db;
use app::server::setup_server;
use app::services::Services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config: Arc<AppConfig> =
        Arc::new(confy::load_path("config/app-local.yaml").expect("Failed to load config"));
    let db = Arc::new(setup_db(config.db.clone()).await);
    let services = Arc::new(Services::create(db.clone()));
    setup_server(config, services).await
}
