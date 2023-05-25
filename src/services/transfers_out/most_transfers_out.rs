use rocket::serde::json::{Value, json};

pub fn get_most_transfers_out() -> Value {
    json!({
        "most_transferred_out": [
            {
                "player": "Maguire",
                "form": 0.4,
                "transfers_in": 12154
            },
            {
                "player": "Dier",
                "form": 1.0,
                "transfers_in": 8922
            },
            {
                "player": "Grealish",
                "form": 4.9,
                "transfers_in": 82652
            },
        ]
    })
}