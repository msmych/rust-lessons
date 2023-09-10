use std::sync::Arc;

use common::{add_entity, random_id, Entity};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

#[derive(Serialize, Deserialize)]
pub struct Ingredient {
    id: RecordId,
    name: String,
    owner: Option<String>,
}

impl Ingredient {
    pub fn new(name: &str, owner_id: Option<String>) -> Self {
        Ingredient {
            id: random_id("ingredient"),
            name: name.to_string(),
            owner: owner_id,
        }
    }
}

impl Entity for Ingredient {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Amount {
    Oz(u8),
    Cl(u8),
    Some,
}

pub struct IngredientService {
    db: Arc<Surreal<Client>>,
}

impl IngredientService {
    pub fn create(db: Arc<Surreal<Client>>) -> Self {
        IngredientService { db }
    }

    pub async fn add(&self, ingredient: Ingredient) -> String {
        add_entity(Arc::clone(&self.db), ingredient)
            .await
            .expect("Failed to add ingredient")
    }

    pub async fn get(&self, id: String) -> Result<Option<Ingredient>, surrealdb::Error> {
        self.db.select(("ingredient", id)).await
    }
}
