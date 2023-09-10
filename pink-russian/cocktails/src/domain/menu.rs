use std::sync::Arc;

use common::{add_entity, random_id, Entity};
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

impl Entity for Menu {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

pub struct MenuService {
    db: Arc<Surreal<Client>>,
}

impl MenuService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        MenuService { db }
    }

    pub async fn add(&self, menu: Menu) -> String {
        add_entity(Arc::clone(&self.db), menu)
            .await
            .expect("Failed to add menu")
    }

    pub async fn get(&self, id: String) -> Result<Option<Menu>, surrealdb::Error> {
        self.db.select(("menu", id)).await
    }
}
