#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let fidget = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("keep_alive", 3))
        .merge(("address", "127.0.0.1"));
    rocket::custom(fidget).mount("/", routes![index])
}
