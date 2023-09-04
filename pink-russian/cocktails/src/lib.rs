use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

pub mod menu;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordId {
    pub id: Thing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    id: Thing,
    pub name: String,
}

impl Account {
    pub fn new(name: &str) -> Self {
        Account {
            id: Thing {
                tb: String::from("account"),
                id: Id::rand(),
            },
            name: String::from(name),
        }
    }
}

pub struct AccountService {
    db: Surreal<Client>,
}

impl AccountService {
    pub fn create(db: Surreal<Client>) -> Self {
        AccountService { db }
    }

    pub async fn add(&self, account: Account) -> Result<String, surrealdb::Error> {
        self.db
            .create("account")
            .content(account.clone())
            .await
            .and_then(|r: RecordId| Ok(r.id.to_string()))
    }

    pub async fn get(&self, id: String) -> Result<Option<Account>, surrealdb::Error> {
        self.db.select(("account", id)).await
    }
}
