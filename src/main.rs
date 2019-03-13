#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
use crate::routes::{get, static_files};
use std::env;

fn rocket() -> rocket::Rocket {
    println!("{}", env::current_dir().unwrap().display());
    rocket::ignite().mount("/", routes![static_files::file, get::index,])
}

fn main() {
    println!("hello world");
    rocket().launch();
}
