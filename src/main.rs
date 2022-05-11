#[macro_use]
extern crate rocket;

mod services;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount(
        "/crates",
        routes![
            services::crates::get_crate_downloads,
            services::crates::get_crate_version_downloads,
            services::crates::get_crate_version
        ],
    )
}
