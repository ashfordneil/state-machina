#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;
extern crate itertools;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate serde_json;

mod nfa;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
