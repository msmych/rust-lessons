use std::sync::Arc;

use common::{random_id, repo::Repo, Entity};
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
    repo: Repo,
}

impl MenuService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        MenuService {
            repo: Repo::create(db, "menu"),
        }
    }

    pub async fn add(&self, menu: Menu) -> String {
        self.repo.add_entity(menu).await
    }

    pub async fn get(&self, id: String) -> Menu {
        self.repo.get_entity(&id).await
    }
}
