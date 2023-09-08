use std::sync::Arc;

use common::random_id;
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

pub struct AccountService {
    db: Arc<Surreal<Client>>,
}

impl AccountService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        AccountService { db }
    }

    pub async fn add(&self, account: Account) -> Result<String, surrealdb::Error> {
        self.db
            .create("account")
            .content(account)
            .await
            .and_then(|v: Vec<Account>| Ok(v.first().expect("msg").id.id.to_string()))
    }

    pub async fn get(&self, id: String) -> Result<Option<Account>, surrealdb::Error> {
        self.db.select(("account", id)).await
    }
}
