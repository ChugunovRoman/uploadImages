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

use rocket::http::Status;
use rocket::{Request, Response};
use std::io::Cursor;

#[post("/api/v1/images/upload", data = "<data>")]
fn index<'a>(data: Result<DataImages, String>) -> Result<&'a str, Response<'static>> {
  // let path = env::temp_dir().join("uploaded_file");

  match data {
    Ok(d) => {
      for image in d.files {
        println!("FILE: {}", image.name);
      }

      return Ok("Ok");
    }
    Err(err) => {
      return Err(
        Response::build()
          .status(Status::raw(400))
          .sized_body(Cursor::new(err))
          .ok()?,
      )
    }
  }
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
