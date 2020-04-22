pub mod user;

use crate::dtos::{CreateRoomRequest, CreateUserRequest};
// use crate::dtos::CreateUserRequest;
use crate::schema::*;
use crate::utils;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::Queryable;
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

impl User {
    pub fn get_by_id(id: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
        use crate::schema::users::dsl::*;
        // let results: Vec<User> = users.filter(id.eq(id)).load::<User>(conn);
        let results: Result<Vec<User>, Error> = users.filter(id.eq(id)).load::<User>(conn);
        match results {
            Ok(mut list) => {
                if list.len() < 1 {
                    return Err("No user found");
                } else {
                    return Ok(list.remove(0));
                }
            }
            Err(reason) => {
                // Log the error
                return Err("Error while fetching results");
            }
        }
    }

    pub fn create_from_request(
        create_request: &CreateUserRequest,
        conn: &MysqlConnection,
    ) -> Result<(), &'static str> {
        let result = diesel::insert_into(users::table)
            .values(create_request)
            .execute(conn);

        match result {
            Ok(_) => {
                return Ok(());
            }
            Err(reason) => {
                // log the error.
                return Err("could not create the user");
            }
        }
    }

    pub fn get_by_email(email_from_request: &str, conn: &MysqlConnection) -> Option<User> {
        use crate::schema::users::dsl::*;
        let results = users
            .filter(email.eq(email_from_request))
            .limit(1)
            .load::<User>(conn);
        match results {
            Ok(mut list) => {
                if list.len() < 1 {
                    return None;
                } else {
                    return Some(list.remove(0));
                }
            }
            Err(reason) => {
                // log the error
                return None;
            }
        }
    }
}

// TODO: think if we can not export every field as public
#[derive(Queryable, Debug)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub creator_user_id: i32,
    pub url_identifier: String,
    pub is_public: bool,
}

impl Room {

    pub fn get_all(conn: &diesel::MysqlConnection) -> Result<Vec<Room>, &'static str> {
        use crate::schema::rooms::dsl::*;
        let results = rooms
                        .load::<Room>(conn);
        match results {
            Ok(list) => {
                Ok(list)
            },
            Err(reason) => {
                println!("failed: {}", reason);
                return Err("some problems");
            }
        }
    }

    pub fn get_dummy() -> Room {
        Room {
            id: 1,
            name: "kumar'2".to_string(),
            creator_user_id: 2,
            is_public: true,
            url_identifier: "/cod/kumars".to_string(),
        }
    }

    pub fn create_from_request(
        request: CreateRoomRequest,
        conn: &MysqlConnection,
    ) -> Result<(), &'static str> {
        use crate::schema::rooms::dsl::*;
        let path = utils::generate_v4_base64_uuid();
        let room_name = request.room_name.unwrap_or(String::new());
        let is_public_room = request.is_public.unwrap_or(false);
        match diesel::insert_into(rooms)
            .values((
                creator_user_id.eq(request.creator_user_id),
                url_identifier.eq(path),
                is_public.eq(is_public_room),
                name.eq(room_name),
            ))
            .execute(conn)
        {
            Ok(_) => {
                return Ok(());
            }
            Err(reason) => {
                println!("error creating room: {}", reason);
                return Err("could not create room");
            }
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
