use std::sync::Arc;

use common::{random_id, repo::Repo, Entity};
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
    repo: Repo,
}

impl AccountService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        Self {
            repo: Repo::create(db, "account"),
        }
    }

    pub async fn add(&self, account: Account) -> String {
        self.repo.add_entity(account).await
    }

    pub async fn get(&self, id: String) -> Account {
        self.repo.get_entity(&id).await
    }
}
