#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod nfa;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
