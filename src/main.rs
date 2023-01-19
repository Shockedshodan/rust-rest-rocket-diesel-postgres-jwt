#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod db;
mod post;
pub mod schema;
mod user;

use db::*;
use std::{thread, time};

pub type RocketBuild = rocket::Rocket<rocket::Build>;
#[get("/")]
async fn ok() -> &'static str {
    "Ok"
}

#[get("/sleep/<seconds>")]
async fn sleep(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}
#[rocket::main]
async fn main() {
    let mut r = rocket::build()
        .attach(PostgresDatabaseConnection::fairing())
        .mount("/", routes![ok, sleep]);

    r = user::mount(r);
    r = post::mount(r);

    let _result = r.launch().await;
    println!("Rocket: shutdown.");
}
