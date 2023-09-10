extern crate surrealdb;

use std::sync::Arc;

use serde::{de::DeserializeOwned, ser::Serialize, Deserialize};
use surrealdb::{engine::remote::ws::Client, opt::RecordId, sql::Id, Surreal};

pub trait Entity {
    fn id(&self) -> &RecordId;
}

pub async fn add_entity(
    db: Arc<Surreal<Client>>,
    entity: impl Entity + Serialize,
) -> Result<String, surrealdb::Error> {
    db.create(entity.id().tb.as_str())
        .content(entity)
        .await
        .and_then(|v: Vec<EntityResponse>| Ok(v.first().expect("msg").id.id.to_string()))
}

pub async fn get_entity<T: Entity + DeserializeOwned>(
    db: Arc<Surreal<Client>>,
    id: &RecordId,
) -> T {
    db.select((&id.tb, id.id.to_string()))
        .await
        .expect(format!("Failed to fetch {}", &id).as_str())
        .expect(format!("Not found {}", &id).as_str())
}

pub fn random_id(table: &str) -> RecordId {
    RecordId {
        tb: String::from(table),
        id: Id::rand(),
    }
}

pub fn record_id(table: &str, id: &str) -> RecordId {
    RecordId {
        tb: String::from(table),
        id: Id::from(id),
    }
}

#[derive(Deserialize)]
struct EntityResponse {
    id: RecordId,
}
