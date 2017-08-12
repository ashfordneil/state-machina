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
use nfa::*;

mod nfa;

#[get("/")]
fn index() -> &'static str {
    //Render home page
    "Hello, World!"
}

// The type to represent the ID of a message.
type ID = usize;

#[derive(Serialize, Deserialize)]
struct Message {
        id: Option<ID>,
            contents: String
}

// curl -X POST --data '{"id": 1, "contents":"asdf"}' http://localhost:8000/test -H "Content-Type: application/json"
#[post("/test", format="application/json", data="<data>")]
fn test_thing(data: Json<Message>) -> &'static str {
    "it works!"
}

#[post("/submit", format="application/json", data="<data>")]
fn submit_nfa(data: Json<Nfa<Unsanitary>>) -> &'static str {
    "it works!"
}


fn main() {
    rocket::ignite().mount("/", routes![index, test_thing, submit_nfa]).launch();
}
