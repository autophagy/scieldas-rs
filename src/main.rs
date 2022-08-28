#[macro_use]
extern crate rocket;

mod scieldas;
mod services;
mod utils;

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
    rocket::build()
        .mount("/", routes![index, health])
        .mount("/crates", services::crates::routes())
        .mount("/licenses", services::licenses::routes())
        .mount("/codestyles", services::codestyles::routes())
}
