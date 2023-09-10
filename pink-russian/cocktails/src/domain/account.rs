use std::sync::Arc;

use common::{add_entity, get_entity, random_id, record_id, Entity};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize)]
pub struct Account {
    id: RecordId,
    pub name: String,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Account {
            id: random_id("account"),
            name: String::from(name),
        }
    }
}

impl Entity for Account {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

pub struct AccountService {
    db: Arc<Surreal<Client>>,
}

impl AccountService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        AccountService { db }
    }

    pub async fn add(&self, account: Account) -> String {
        add_entity(Arc::clone(&self.db), account)
            .await
            .expect("Failed to add account")
    }

    pub async fn get(&self, id: String) -> Account {
        get_entity(Arc::clone(&self.db), &record_id("account", &id)).await
    }
}
