use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, Surreal};

use crate::{Entity, EntityResponse};

pub struct Repo {
    pub db: Arc<Surreal<Client>>,
    table: String,
}

impl Repo {
    pub fn create(db: Arc<Surreal<Client>>, table: &str) -> Self {
        Self {
            db,
            table: table.to_string(),
        }
    }

    pub async fn add_entity(&self, entity: impl Entity + Serialize) -> RecordId {
        let select = self.db
        .create(entity.id().tb.as_str())
        .content(&entity);
        let v: Vec<EntityResponse> = select.await
            .expect(&format!("Failed to add {}", &entity.id()));
        v.first().expect(&format!("Failed to add {}", &entity.id())).id.clone()
    }

    pub async fn get_entity<T: Entity + DeserializeOwned>(&self, id: String) -> T {
        let select = self.db.select((&self.table, id.to_string()));
        select.await
            .expect(&format!("Failed to fetch {}:{}", self.table, &id))
            .expect(&format!("Not found {}:{}", self.table, &id))
    }
}
