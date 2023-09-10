use common::{add_entity, random_id, Entity};
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
    db: Arc<Surreal<Client>>,
}

impl RecipeService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        RecipeService { db }
    }

    pub async fn add(&self, recipe: Recipe) -> String {
        add_entity(Arc::clone(&self.db), recipe)
            .await
            .expect("Failed to add recipe")
    }

    pub async fn get(&self, id: String) -> Result<Option<Recipe>, surrealdb::Error> {
        self.db.select(("recipe", id)).await
    }
}
