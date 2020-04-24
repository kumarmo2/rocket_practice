pub mod user;

use crate::dtos::{CreateRoomRequest, CreateUserRequest};
// use crate::dtos::CreateUserRequest;
use crate::schema::*;
use crate::utils;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Queryable, Insertable, QueryableByName};
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::databases::diesel;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub email: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: String::new(),
            email: String::new(),
            age: 0,
        }
    }

    // pb fn create_user(conn: create_request: CreateUserRequest)
}

// impl User {
//     pub fn get_by_id(id: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
//         use crate::schema::users::dsl::*;
//         // let results: Vec<User> = users.filter(id.eq(id)).load::<User>(conn);
//         let results: Result<Vec<User>, Error> = users.filter(id.eq(id)).load::<User>(conn);
//         match results {
//             Ok(mut list) => {
//                 if list.len() < 1 {
//                     return Err("No user found");
//                 } else {
//                     return Ok(list.remove(0));
//                 }
//             }
//             Err(reason) => {
//                 // Log the error
//                 return Err("Error while fetching results");
//             }
//         }
//     }

//     pub fn create_from_request(
//         create_request: &CreateUserRequest,
//         conn: &MysqlConnection,
//     ) -> Result<(), &'static str> {
//         let result = diesel::insert_into(users::table)
//             .values(create_request)
//             .execute(conn);

//         match result {
//             Ok(_) => {
//                 return Ok(());
//             }
//             Err(reason) => {
//                 // log the error.
//                 return Err("could not create the user");
//             }
//         }
//     }

//     pub fn get_by_email(email_from_request: &str, conn: &MysqlConnection) -> Option<User> {
//         use crate::schema::users::dsl::*;
//         let results = users
//             .filter(email.eq(email_from_request))
//             .limit(1)
//             .load::<User>(conn);
//         match results {
//             Ok(mut list) => {
//                 if list.len() < 1 {
//                     return None;
//                 } else {
//                     return Some(list.remove(0));
//                 }
//             }
//             Err(reason) => {
//                 // log the error
//                 return None;
//             }
//         }
//     }
// }

// TODO: think if we can not export every field as public
#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub creator_user_id: i32,
    pub url_identifier: String,
    pub is_public: bool,
}

impl Room {

    pub fn get_dummy() -> Room {
        Room {
            id: 1,
            name: "kumar'2".to_string(),
            creator_user_id: 2,
            is_public: true,
            url_identifier: "/cod/kumars".to_string(),
        }
    }
}

// I have implemented Responder trait just for illustraition purpose,
// We should not expose models directly.
impl<'r> Responder<'r> for Room {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!(
                "Room' name: {}, id: {}, path: {}",
                self.name, self.id, self.url_identifier
            )))
            .ok()
    }
}

// TODO: need to figure out how to have field names different than the db column names.
// #[derive(Debug, Queryable, Insertable)]
#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "roomsubscribers"]
pub struct RoomSubscriber {
    // pub id: i32,
    pub member_id: i32,
    pub room_id: i32,
}

impl RoomSubscriber {
    pub fn new(member_id: i32, room_id: i32) -> Self {
        RoomSubscriber {
            // id,
            member_id,
            room_id,
        }
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
}

pub struct CounterWrapper {
    counter: Arc<Mutex<u32>>,
}

pub struct CustomKey(pub String);

impl CounterWrapper {
    pub fn increment(&self) {
        let cloned = Arc::clone(&self.counter);
        let mut counter = cloned.lock().unwrap();
        *counter = *counter + 1;
        println!("new counter: {}", *counter);
    }
}

impl Default for CounterWrapper {
    fn default() -> Self {
        CounterWrapper {
            counter: Arc::new(Mutex::new(0)),
        }
    }
}

#[database("mysql")]
pub struct MySqlDb(diesel::MysqlConnection);
