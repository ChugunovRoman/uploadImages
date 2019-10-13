#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin, custom_attribute)]

mod guards;
mod logger;
mod utils;

#[macro_use]
extern crate rocket;
extern crate log;
extern crate multipart;

use guards::DataImages;

use rocket::{Data, Request};
use std::{env, io};

#[post("/api/v1/images/upload", data = "<data>")]
fn index(data: DataImages) -> io::Result<String> {
  // let path = env::temp_dir().join("uploaded_file");

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
