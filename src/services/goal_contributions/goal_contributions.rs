use rocket::serde::json::{Value, json};

pub fn get_goal_contributors() -> Value {
    json!({
        "goal_contributors": [
            {
                "player": "Haaland",
                "goals": 36,
                "assist": 8
            },
            {
                "player": "Kane",
                "goals": 28,
                "assist": 12
            },
            {
                "player": "Salah",
                "goals": 18,
                "assist": 14
            },
        ]
    })
}