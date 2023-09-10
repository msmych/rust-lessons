use serde::Deserialize;
use surrealdb::{opt::RecordId, sql::Id};

extern crate surrealdb;

pub mod repo;

pub trait Entity {
    fn id(&self) -> &RecordId;
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
