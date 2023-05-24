use rocket::serde::json:: {json, Value};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    String::from("Hello, world!")
}

#[get("/in")]
fn get_most_transfers_in() -> Value {
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

#[get("/out")]
fn get_most_transfers_out() -> Value {
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/transfers", routes![get_most_transfers_in])
        .mount("/transfers", routes![get_most_transfers_out])
}
