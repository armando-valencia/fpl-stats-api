#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};

pub mod routes;
pub mod services;

use routes::index::index;
use routes::transfers::{get_transfers_in, get_transfers_out};

#[catch(500)]
fn not_found() -> Value {
    json!({
        "status": "Server error",
        "reason": "Something went wrong. Please try again."
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, get_transfers_in, get_transfers_out])
        .register("/", catchers![not_found])
}
