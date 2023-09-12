use common::{random_id, repo::Repo, Entity};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use super::ingredient::Amount;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    id: RecordId,
    name: String,
    ingredients: HashMap<String, Amount>,
    instruction: String,
}

impl Recipe {
    pub fn new(name: &str, ingredients: HashMap<String, Amount>, instruction: &str) -> Self {
        Recipe {
            id: random_id("recipe"),
            name: name.to_string(),
            ingredients,
            instruction: instruction.to_string(),
        }
    }
}

impl Entity for Recipe {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

pub struct RecipeService {
    repo: Repo,
}

impl RecipeService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        Self {
            repo: Repo::create(db, "recipe"),
        }
    }

    pub async fn add(&self, recipe: Recipe) -> RecordId {
        self.repo.add_entity(recipe).await
    }

    pub async fn get(&self, id: String) -> Recipe {
        self.repo.get_entity(&id).await
    }
}
