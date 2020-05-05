#![feature(proc_macro_hygiene, decl_macro)]
// TODO: remove after developement.
// #![allow(warnings)]

#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;
use rocket::fairing::AdHoc;
use rocket::http::Header;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use manager::RabbitMqManager;

// mod routes::home;

mod business;
mod dal;
mod dtos;
mod error_catchers;
mod models;
mod routes;
mod schema;
mod utils;
mod view_models;

use routes::message;
use routes::room;
use routes::user;

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::home::index, routes::home::dummy])
        .mount(
            "/api/users",
            routes![
                user::get,
                user::create,
                user::register_user_event_queue,
                user::cors_for_register_events_endpoint
            ],
        )
        .mount(
            "/api/rooms",
            routes![
                room::add_members,
                room::create,
                room::get,
                room::get_all,
                room::get_room_info,
                room::get_room_member_ids,
                room::get_room_client_queues,
            ],
        )
        .mount("/api/messages", routes![message::create, message::get])
        .mount("/public", StaticFiles::from("./public"))
        .manage(models::CounterWrapper::default())
        // TODO: read from the config.
        .manage(RabbitMqManager::new(
            "amqp://guest:guest@127.0.0.1:5672/%2f",
        ))
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
        .attach(Template::fairing())
        .attach(AdHoc::on_response("cors_respone", |_, res| {
            // TODO: only set CORS headers for selected endpoints and not for all
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
        .register(catchers![error_catchers::bad_request])
        .launch();
}
