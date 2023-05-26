use rocket::serde::json::{Value, json};

pub fn get_mr_reliable() -> Value {
    json!({
        "mr_reliable": [
            {
                "player": "Ward-Prowse",
                "minutes": 12154
            },
            {
                "player": "Fernandes",
                "minutes": 8922
            },
            {
                "player": "Saka",
                "minutes": 8265
            },
        ]
    })
}