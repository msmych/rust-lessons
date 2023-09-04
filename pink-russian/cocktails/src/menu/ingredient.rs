use std::sync::Arc;

use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    opt::RecordId,
    sql::{Id, Thing},
    Surreal,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    id: RecordId,
    name: String,
    owner: Option<String>,
}

impl Ingredient {
    pub fn new(name: &str, owner_id: Option<String>) -> Self {
        Ingredient {
            id: Thing {
                tb: String::from("ingredient"),
                id: Id::rand(),
            },
            name: name.to_string(),
            owner: owner_id,
        }
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

    pub async fn add(&self, ingredient: Ingredient) -> Result<String, surrealdb::Error> {
        self.db
            .create("ingredient")
            .content(ingredient)
            .await
            .and_then(|v: Vec<Ingredient>| Ok(v.first().expect("msg").id.id.to_string()))
    }

    pub async fn get(&self, id: String) -> Result<Option<Ingredient>, surrealdb::Error> {
        self.db.select(("ingredient", id)).await
    }
}
