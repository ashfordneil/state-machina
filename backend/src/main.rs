#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde;

#[macro_use]
extern crate serde_derive;

fn main() {
    rocket::ignite().launch();
}
