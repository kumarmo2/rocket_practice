use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

pub struct User {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: 0,
            name: String::new(),
            age: 0,
        }
    }
}

// TODO: think if we can not export every field as public
pub struct Room {
    pub id: u32,
    pub name: String,
    pub creator_user_id: u32,
    pub is_public: bool,
    pub path: String,
}

impl Room {
    pub fn get_dummy() -> Room {
        Room {
            id: 1,
            name: "kumar'2".to_string(),
            creator_user_id: 2,
            is_public: true,
            path: "/cod/kumars".to_string(),
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
                self.name, self.id, self.path
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
