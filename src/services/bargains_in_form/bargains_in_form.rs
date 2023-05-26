use rocket::serde::json::{Value, json};

pub fn get_bargains_in_form() -> Value {
    json!({
        "bargains_in_form(": [
            {
                "player": "Awoniyi",
                "cost": 5.2,
                "form": 8.1,
                "goals": 12,
                "assists": 4,
                "selected_by_percent": 2.3
            },
            {
                "player": "Estupinan",
                "cost": 5.1,
                "form": 7.6,
                "goals": 2,
                "assists": 5,
                "selected_by_percent": 6.8
            },
            {
                "player": "Mbeumo",
                "cost": 5.6,
                "form": 6.2,
                "goals": 9,
                "assists": 10,
                "selected_by_percent": 3.8
            }
        ]
    })
}