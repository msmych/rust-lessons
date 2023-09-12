use std::sync::Arc;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::config::DbConfig;

pub async fn setup_db(config: Arc<DbConfig>) -> Surreal<Client> {
    let db = Surreal::new::<Ws>(&config.address)
        .await
        .expect("Failed to connect to DB");
    db.signin(Root {
        username: &config.username,
        password: &config.password,
    })
    .await
    .expect("Failed to sign in to DB");
    db.use_ns(&config.ns)
        .use_db(&config.db)
        .await
        .expect("Failed to select namespace or DB");
    db
}
