#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{Data, Request};
use std::{env, io};

#[post("/api/v1/images/upload", format = "plain", data = "<data>")]
fn index(data: Data) -> io::Result<String> {
  let path = env::temp_dir().join("uploaded_file");

  data.stream_to_file(&path)?;

  Ok("Ok".to_string())
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
