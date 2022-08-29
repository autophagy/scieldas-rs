#[macro_use]
extern crate rocket;

mod scieldas;
mod services;
mod utils;

use reqwest::Client;

#[get("/")]
fn index() -> &'static str {
    "Scieldas."
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    let client = Client::builder().user_agent("scieldas").build().unwrap();

    rocket::build()
        .manage(client)
        .mount("/", routes![index, health])
        .mount("/crates", services::crates::routes())
        .mount("/github", services::github::routes())
        .mount("/licenses", services::licenses::routes())
        .mount("/codestyles", services::codestyles::routes())
}
