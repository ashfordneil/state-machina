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

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, submit_nfa, files])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;

    use serde_json;
    use automata::*;

    /// Test home page ("/" or "/index.html")
    #[test]
    fn test_index() {
        let rocket = rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
        assert!(response.body_string().unwrap().contains("<noscript>You need to enable JavaScript to run this app.</noscript>"));
        //assert!(response.headers().get_one("X-Special").is_some());

        let response = client.get("/index.html").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::HTML));
    }

    #[test]
    fn test_not_found() {
        let rocket = rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.get("/asdfasdfasdf").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    /// Test POSTing of NFA in JSON
    #[test]
    fn test_submit_nfa() {
        let rocket = rocket();
        let client = Client::new(rocket).expect("valid rocket instance");

        //Test good JSON
        let input = r#"{
            "start": "1",
            "alphabet": ["a", "b"],
            "nodes": {
                "1": {
                    "a": ["1", "2"],
                    "b": ["1"]
                },
                "2": {
                    "a": ["3"],
                    "b": ["3"]
                },
                "3": {
                    "a": ["1"],
                    "b": ["2"]
                }
            },
            "final_states": ["3"]
        }"#;

        let mut response = client.post("/submit")
            .body(input)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        //Convert JSON to DFA
        let body_string: &str = &(response.body_string()).unwrap();
        let dfa_thing: Dfa = serde_json::from_str(body_string).unwrap();
    }


    #[test]
    fn test_empty_nfa() {
        let rocket = rocket();
        let client = Client::new(rocket).expect("valid rocket instance");
        let response = client.post("/submit")
            .body("{}")
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
    }
    
}
