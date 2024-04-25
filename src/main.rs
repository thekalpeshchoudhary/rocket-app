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
fn view_rustaeans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id":id, "name":"Kalpesh", "email":"kalpesh@gmail.com"})
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

#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id":id, "name":"Max", "email":"max@example.com"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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
            catchers![not_found, unauthorized, internal_server_error],
        )
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
