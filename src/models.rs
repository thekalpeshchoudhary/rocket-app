use crate::schema::rustaceans;
use diesel::{prelude::Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = rustaceans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Rustaceans {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
