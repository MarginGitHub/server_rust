#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate time;

use rocket_contrib::{Template};

mod router;

pub fn run() {
    rocket::ignite()
    .attach(Template::fairing())
    .mount("/", router::routes())
    .launch();
}
