use std::sync::Arc;

use common::random_id;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize)]
pub struct Menu {
    id: RecordId,
    account_id: String,
    name: String,
}

impl Menu {
    pub fn new(account_id: String, name: &str) -> Self {
        Menu {
            id: random_id("menu"),
            account_id,
            name: name.to_string(),
        }
    }
}

pub struct MenuService {
    db: Arc<Surreal<Client>>,
}

impl MenuService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        MenuService { db }
    }

    pub async fn add(&self, menu: Menu) -> Result<String, surrealdb::Error> {
        self.db
            .create("menu")
            .content(menu)
            .await
            .and_then(|v: Vec<Menu>| Ok(v.first().expect("msg").id.id.to_string()))
    }

    pub async fn get(&self, id: String) -> Result<Option<Menu>, surrealdb::Error> {
        self.db.select(("menu", id)).await
    }
}
