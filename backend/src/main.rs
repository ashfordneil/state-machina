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

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

mod automata;
use automata::{Nfa, Dfa, Unsanitary};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("../frontend/build/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../frontend/build/").join(file)).ok()
}

#[post("/submit", format="application/json", data="<data>")]
fn submit_nfa(data: Json<Nfa<Unsanitary>>) -> Json<Dfa> {
    Json(data.into_inner().check().unwrap().make_deterministic())
}


fn main() {
    rocket::ignite().mount("/", routes![index, submit_nfa, files]).launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_one() {
        let rocket = rocket::ignite();
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        //assert_eq!(response.status(), Status::Ok);
        //assert_eq!(response.content_type(), Some(ContentType::Plain));
        //assert!(response.headers().get_one("X-Special").is_some());
        //assert_eq!(response.body_string(), Some("Expected Body.".into()));
    }
}
