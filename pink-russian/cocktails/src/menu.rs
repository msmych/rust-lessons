use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

pub mod ingredient;
pub mod recipe;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    id: Thing,
    account_id: String,
    name: String,
}

impl Menu {
    pub fn new(account_id: String, name: &str) -> Self {
        Menu {
            id: Thing {
                tb: String::from("menu"),
                id: Id::rand(),
            },
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
            .content(menu.clone())
            .await
            .and_then(|v: Vec<Menu>| Ok(v.first().expect("msg").id.id.to_string()))
    }

    pub async fn get(&self, id: String) -> Result<Option<Menu>, surrealdb::Error> {
        self.db.select(("menu", id)).await
    }
}
