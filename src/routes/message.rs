use crate::business::message;
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::MessageCreateRequest;
use crate::models::MySqlDb;

use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

use manager::RabbitMqManager;

use diesel::result::Error;

#[post("/", data = "<request_json>")]
pub fn create(
    api_key: ApiKey,
    rabbit: State<RabbitMqManager>,
    request_json: Json<MessageCreateRequest>,
    conn: MySqlDb,
) -> Status {
    match validate_create_message_request(&request_json) {
        Some(reason) => return Status::new(400, reason),
        None => {}
    }
    match message::create(&request_json, &conn, &rabbit) {
        Some(id) => {
            println!("messaged id: {}", id);
            return Status::Created;
        }
        None => {
            return Status::InternalServerError;
        }
    }
}

fn validate_create_message_request(request: &MessageCreateRequest) -> Option<&'static str> {
    if request.content.len() < 1 {
        Some("content cannot be empty")
    } else if request.room_id < 1 {
        Some("invalid room id")
    } else if (request.sender_id < 1) {
        Some("invalid sender id")
    } else {
        None
    }
}
