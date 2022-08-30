#[macro_use]
extern crate rocket;

mod scieldas;
mod services;
mod utils;

use reqwest::Client;
use std::env;

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
    let mut opt = usvg::Options::default();

    match env::var("FONTS_DIR") {
        Ok(dir) => opt.fontdb.load_fonts_dir(dir),
        Err(_) => opt.fontdb.load_system_fonts(),
    };

    rocket::build()
        .manage(client)
        .manage(opt)
        .mount("/", routes![index, health])
        .mount("/crates", services::crates::routes())
        .mount("/github", services::github::routes())
        .mount("/licenses", services::licenses::routes())
        .mount("/codestyles", services::codestyles::routes())
}
