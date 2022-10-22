#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> &'static str {
  "Hello darkness my old friend"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index])
}
