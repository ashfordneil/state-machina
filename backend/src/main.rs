#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;
extern crate itertools;

#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

#[cfg(test)]
extern crate serde_json;

use rocket_contrib::Json;

mod automata;
use automata::{Nfa, Dfa, Unsanitary};

#[get("/")]
fn index() -> &'static str {
    //Render home page
    "Hello, World!"
}

#[post("/submit", format="application/json", data="<data>")]
fn submit_nfa(data: Json<Nfa<Unsanitary>>) -> Json<Dfa> {
    Json(data.into_inner().check().unwrap().make_deterministic())
}


fn main() {
    rocket::ignite().mount("/", routes![index, submit_nfa]).launch();
}
