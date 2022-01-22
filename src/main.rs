#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] imports rocket across the whole project. It can be replaced with use rocket instead.
// In that case we will have to replace every instance of rocket eg rocket::get
#[macro_use] extern crate rocket;

mod paste_id;
mod file_ops;

#[get("/")]
fn index() -> &'static str{
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    rocket::ignite().mount("/", routes![index, file_ops::upload, file_ops::retrieve]).register(catchers![not_found]).launch();
}
