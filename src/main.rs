#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;

#[get("/")]
fn index() -> &'static str {
  "Hello Rocket!"
}

#[catch(404)]
fn not_found(request: &Request) -> String {
  format!("Path {} is not a valid path :C", request.uri())
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index])
    .register(catchers![not_found])
    .launch();
}
