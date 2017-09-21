use rocket::response::NamedFile;
use rocket::Route;

use std::path::{Path, PathBuf};
use std::io;
use std::collections::HashMap;
use time::now;

pub fn routes() -> Vec<Route> {
    routes![file, index]
}

#[get("/static/<file..>")]
fn file(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("static/").join(file))
}

#[get("/")]
fn index() -> ::Template {
    let mut c = HashMap::new();
    let now = now();
    c.insert("year", now.tm_year + 1900);
    c.insert("month", now.tm_mon);
    c.insert("day", now.tm_mday);
    c.insert("hour", now.tm_hour);
    c.insert("minute", now.tm_min);
    c.insert("second", now.tm_sec);
    ::Template::render("index", c)
}