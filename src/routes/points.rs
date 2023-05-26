use rocket::serde::json::Value;
use crate::services;

// Bargains in form
#[get("/bargains")]
pub fn get_bargains_in_form() -> Value {
    services::bargains_in_form::bargains_in_form::get_bargains_in_form()
}

// Goal Contribution Leaders
#[get("/contributors")]
pub fn get_goal_contributors() -> Value {
    services::goal_contributions::goal_contributions::get_goal_contributors()
}

// Mr Reliable
#[get("/reliable")]
pub fn get_mr_reliable() -> Value {
    services::mr_reliable::mr_reliable::get_mr_reliable()
}
