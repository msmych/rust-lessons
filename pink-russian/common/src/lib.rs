extern crate surrealdb;

use surrealdb::{opt::RecordId, sql::Id};

pub fn random_id(table: &str) -> RecordId {
    RecordId {
        tb: String::from(table),
        id: Id::rand(),
    }
}
