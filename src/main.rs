use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/health")]
fn health() -> &'static str {
    "service is online"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health])
}
