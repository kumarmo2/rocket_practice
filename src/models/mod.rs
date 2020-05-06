pub mod user;

// use crate::dtos::CreateUserRequest;
use crate::schema::*;
use diesel::{Queryable, QueryableByName};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_contrib::databases::diesel;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
    pub email: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: String::new(),
            email: String::new(),
            age: Some(0),
            password: String::new(),
        }
    }
}

#[derive(Queryable, QueryableByName, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub creator_user_id: i32,
    pub url_identifier: String,
    pub is_public: bool,
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

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "messages"]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "queues"]
pub struct Queue {
    pub id: i32,
    pub user_id: i32,
    pub queue_name: String,
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
