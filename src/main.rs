#![feature(proc_macro_hygiene, decl_macro)]

// TODO: remove after developement.
#![allow(warnings)]

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;

// mod routes::home;

mod business;
mod dtos;
mod error_catchers;
mod models;
mod routes;
mod schema;
mod utils;
mod dal;

use routes::home;
use routes::room;
use routes::user;
use routes::message;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::home::index,])
        .mount(
            "/api/users",
            routes![
                user::big_hello,
                // user::get_user_by_id,
                user::user_authorized_endpoint,
                user::create,
            ],
        )
        .mount("/api/rooms", routes![room::add_members,room::create, room::get, room::get_all ])
        .mount("/api/messages", routes![message::create])
        .mount("/public", StaticFiles::from("./static"))
        .manage(models::CounterWrapper::default())
        .attach(AdHoc::on_attach("config_fairing", |rocket| {
            let val = rocket
                .config()
                .get_str("custom_key")
                .unwrap_or("defaultValue")
                .to_string();
            println!("config fairing");
            Ok(rocket.manage(models::CustomKey(val)))
        }))
        .attach(models::MySqlDb::fairing())
        .register(catchers![error_catchers::bad_request])
        .launch();
}
