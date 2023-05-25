use rocket::serde::json::{Value, json};

pub fn get_most_transfers_in() -> Value {
    json!({
        "most_transferred_in": [
            {
                "player": "Saka",
                "form": 6.3,
                "transfers_in": 192154
            },
            {
                "player": "Salah",
                "form": 5.2,
                "transfers_in": 84922
            },
            {
                "player": "Grealish",
                "form": 4.9,
                "transfers_in": 82652
            },
        ]
    })
}