use std::sync::Arc;

use common::{random_id, record_id, repo::Repo, Entity};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    id: RecordId,
    name: String,
    owner: Option<String>,
}

impl Ingredient {
    pub fn new(name: &str, owner_id: &str) -> Self {
        Ingredient {
            id: random_id("ingredient"),
            name: name.to_string(),
            owner: Some(owner_id.to_string()),
        }
    }

    pub fn default(id: &str, name: &str) -> Self {
        Ingredient {
            id: record_id("ingredient", id),
            name: name.to_string(),
            owner: None,
        }
    }
}

impl Entity for Ingredient {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
pub enum Amount {
    Oz(u8),
    Cl(u8),
    Some,
}

pub struct IngredientService {
    repo: Repo,
}

impl IngredientService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        IngredientService {
            repo: Repo::create(db, "ingredient"),
        }
    }

    pub async fn add(&self, ingredient: Ingredient) -> String {
        self.repo.add_entity(ingredient).await
    }

    pub async fn get(&self, id: String) -> Ingredient {
        self.repo.get_entity(&id).await
    }
}
