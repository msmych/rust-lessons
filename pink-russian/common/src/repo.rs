use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{Entity, EntityResponse};

pub struct Repo {
    pub db: Arc<Surreal<Client>>,
    table: String,
}

impl Repo {
    pub fn create(db: Arc<Surreal<Client>>, table: &str) -> Self {
        Repo {
            db,
            table: String::from(table),
        }
    }

    pub async fn add_entity(&self, entity: impl Entity + Serialize) -> String {
        self.db
            .create(entity.id().tb.as_str())
            .content(&entity)
            .await
            .and_then(|v: Vec<EntityResponse>| {
                Ok(v.first()
                    .expect(&format!("Failed to add {}", &entity.id()))
                    .id
                    .id
                    .to_string())
            })
            .expect(&format!("Failed to add {}", &entity.id()))
    }

    pub async fn get_entity<T: Entity + DeserializeOwned>(&self, id: &str) -> T {
        self.db
            .select((&self.table, String::from(id)))
            .await
            .expect(&format!("Failed to fetch {}", &id))
            .expect(&format!("Not found {}", &id))
    }
}
