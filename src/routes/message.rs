use crate::dtos::{MessageCreateRequest};
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::models::MySqlDb;
use crate::business::message;


use rocket_contrib::json::Json;
use rocket::http::Status;
use diesel::result::Error;


#[post("/", data = "<request_json>")]
pub fn create(api_key: ApiKey, request_json: Json<MessageCreateRequest>, conn: MySqlDb) -> Status {
    match validate_create_message_request(&request_json) {
        Some(reason) => {
            return Status::new(400, reason)
        },
        None => {}
    }
    match message::create(&request_json, &conn) {
        Ok(is_created) => {
            if is_created {
                return Status::Ok;
            } else {
                return Status::InternalServerError;
            }
        },
        Err(reason) => {
            match reason {
                Error::NotFound => {
                    println!("row not found:");
                    return Status::BadRequest;
                },
                _ => {
                    return Status::InternalServerError;
                }
            }
        }
    }
    // if message::create(&request_json, &conn) {
    //     return Status::Ok;
    // } else {

    //     return Status::NotFound;
    // }
}

fn validate_create_message_request(request: &MessageCreateRequest) -> Option<&'static str> {
    if request.content.len() < 1 {
        Some("content cannot be empty")
    } else if request.room_id < 1 {
        Some("invalid room id")
    } else if(request.sender_id < 1) {
        Some("invalid sender id")
    } else {
        None
    }
}