#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, this is the Backend Blog Service"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}