#[macro_use]
extern crate rocket;

mod services;
mod shields;
mod utils;

#[get("/health")]
fn health() -> &'static str {
    "OK"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![health])
        .mount(
            "/crates",
            routes![
                services::crates::get_crate_downloads,
                services::crates::get_crate_version_downloads,
                services::crates::get_crate_version
            ],
        )
        .mount("/licenses", routes![services::licenses::get_license])
}
