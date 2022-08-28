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
        .mount(
            "/crates",
            routes![
                services::crates::get_crate_downloads,
                services::crates::get_crate_version_downloads,
                services::crates::get_crate_version
            ],
        )
        .mount("/licenses", routes![services::licenses::get_license])
        .mount(
            "/codestyles",
            routes![services::codestyles::get_python_style],
        )
}
