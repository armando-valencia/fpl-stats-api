use rocket::serde::json::{Value, json};

pub fn get_most_transfers_out() -> Value {
    json!({
        "most_transferred_out": [
            {
                "player": "Maguire",
                "form": 0.4,
                "transfers_out": 12154
            },
            {
                "player": "Dier",
                "form": 0.7,
                "transfers_out": 8922
            },
            {
                "player": "Mudryk",
                "form": 0.8,
                "transfers_out": 8265
            },
        ]
    })
}