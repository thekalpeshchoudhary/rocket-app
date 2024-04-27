#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{NewRustacean, Rustaceans};
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;
use schema::rustaceans;

#[database("psql")]
struct DbConnection(diesel::PgConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConnection) -> Value {
    db.run(|c| {
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .select(Rustaceans::as_select())
            .load(c)
            .expect("DB Error");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustaeans(id: i32, _auth: BasicAuth, db: DbConnection) -> Value {
    db.run(move |c| {
        let rustacean = rustaceans::table
            .find(id)
            .get_result::<Rustaceans>(c)
            .expect("DB error when selecting rustacean");
        json!(rustacean)
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustaceans(
    _auth: BasicAuth,
    db: DbConnection,
    new_rustacean: Json<NewRustacean>,
) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("DB insertion failed!");
        json!(result)
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(
    id: i32,
    _auth: BasicAuth,
    db: DbConnection,
    rustacean: Json<Rustaceans>,
) -> Value {
    db.run(move |c| {
        let result = diesel::update(rustaceans::table.find(id))
            .set(rustacean.into_inner())
            .execute(c)
            .expect("DB update failed!");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConnection) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
            .expect("DB delete failed!");
        status::NoContent
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized Request")
}

#[catch(500)]
fn internal_server_error() -> Value {
    json!("Internal Server Error")
}

#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unable to Process Request")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_rustaceans,
                view_rustaeans,
                create_rustaceans,
                update_rustaceans,
                delete_rustacean
            ],
        )
        .register(
            "/",
            catchers![
                not_found,
                unauthorized,
                internal_server_error,
                unprocessable_entity
            ],
        )
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
