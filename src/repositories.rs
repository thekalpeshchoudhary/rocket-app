use crate::{
    models::{NewRustacean, Rustaceans},
    schema::rustaceans,
};
use diesel::prelude::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustaceans> {
        rustaceans::table.find(id).get_result::<Rustaceans>(c)
    }

    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustaceans>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(limit)
            .load::<Rustaceans>(c)
    }

    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustaceans> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result::<Rustaceans>(c)
    }

    pub fn update(c: &mut PgConnection, id: i32, rustacean: Rustaceans) -> QueryResult<Rustaceans> {
        diesel::update(rustaceans::table.find(id))
            .set(rustacean)
            .get_result::<Rustaceans>(c)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}
