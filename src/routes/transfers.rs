use rocket::serde::json::Value;
use crate::services;

// Most Transfers In
#[get("/in")]
pub fn get_transfers_in() -> Value {
    services::transfers_in::most_transfers_in::get_most_transfers_in()
}

// Most Transfers Out
#[get("/out")]
pub fn get_transfers_out() -> Value {
    services::transfers_out::most_transfers_out::get_most_transfers_out()
}
