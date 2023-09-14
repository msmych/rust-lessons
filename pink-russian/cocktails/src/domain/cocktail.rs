use common::{random_id, repo::Repo, Entity};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use super::ingredient::Amount;

#[derive(Serialize, Deserialize)]
pub struct Cocktail {
    id: RecordId,
    #[serde(rename = "accountId")]
    account_id: String,
    name: String,
    ingredients: HashMap<String, Amount>,
    recipe: Vec<String>,
}

impl Cocktail {
    pub fn new(name: &str, account_id: &str, ingredients: HashMap<String, Amount>, recipe: Vec<String>) -> Self {
        Self {
            id: random_id("cocktail"),
            account_id: account_id.to_string(),
            name: name.to_string(),
            ingredients,
            recipe,
        }
    }
}

impl Entity for Cocktail {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

pub struct CocktailService {
    repo: Repo,
}

impl CocktailService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        Self {
            repo: Repo::create(db, "cocktail"),
        }
    }

    pub async fn add(&self, cocktail: Cocktail) -> RecordId {
        self.repo.add_entity(cocktail).await
    }

    pub async fn get(&self, id: String) -> Cocktail {
        self.repo.get_entity(id).await
    }
}
