use diesel::Queryable;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Rustaceans {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}
