#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::{NewRustacean, Rustaceans};
use repositories::RustaceanRepository;
use rocket::{
    response::status,
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;

#[database("psql")]
struct DbConnection(diesel::PgConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConnection) -> Value {
    db.run(|c| {
        let rustaceans = RustaceanRepository::find_multiple(c, 1000).expect("DB Error");
        json!(rustaceans)
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustaeans(id: i32, _auth: BasicAuth, db: DbConnection) -> Value {
    db.run(move |c| {
        let rustacean =
            RustaceanRepository::find(c, id).expect("DB error when selecting rustacean");
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
        let result = RustaceanRepository::create(c, new_rustacean.into_inner())
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
        let result =
            RustaceanRepository::update(c, id, rustacean.into_inner()).expect("DB update failed!");
        json!(result)
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, db: DbConnection) -> status::NoContent {
    db.run(move |c| {
        RustaceanRepository::delete(c, id).expect("DB delete failed!");
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
