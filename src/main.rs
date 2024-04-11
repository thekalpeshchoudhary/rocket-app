#[macro_use]
extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]
fn hello() -> Value {
    json!("Hello World")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
