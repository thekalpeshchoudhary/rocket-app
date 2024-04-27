#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use diesel::result::Error::NotFound;
use models::{NewRustacean, Rustaceans};
use repositories::RustaceanRepository;
use rocket::{
    http::Status,
    response::status::{self, Custom},
    serde::json::{json, Json, Value},
};
use rocket_sync_db_pools::database;

#[database("psql")]
struct DbConnection(diesel::PgConnection);

#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConnection) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 1000)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[get("/rustaceans/<id>")]
async fn view_rustaeans(
    id: i32,
    _auth: BasicAuth,
    db: DbConnection,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustaceans(
    _auth: BasicAuth,
    db: DbConnection,
    new_rustacean: Json<NewRustacean>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustaceans(
    id: i32,
    _auth: BasicAuth,
    db: DbConnection,
    rustacean: Json<Rustaceans>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(
    id: i32,
    _auth: BasicAuth,
    db: DbConnection,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        let rustacean_found = RustaceanRepository::find(c, id);
        match rustacean_found {
            Ok(_) => RustaceanRepository::delete(c, id)
                .map(|_| status::NoContent)
                .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string()))),
            Err(_) => Err(Custom(Status::NotFound, json!("Rustacean not found"))),
        }
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
