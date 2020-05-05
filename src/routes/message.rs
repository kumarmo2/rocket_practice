use crate::business::message;
use crate::dtos::request_guards::{AdminAuthorization, ApiKey::ApiKey};
use crate::dtos::MessageCreateRequest;
use crate::models::MySqlDb;

use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

use chat_common_types::dtos as common_dtos;

use manager::RabbitMqManager;

#[post("/", data = "<request_json>")]
pub fn create(
    _api_key: ApiKey,
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

#[get("/<id>")]
pub fn get(
    _admin: AdminAuthorization,
    id: i32,
    conn: MySqlDb,
) -> Result<Json<common_dtos::Message>, Status> {
    if id < 1 {
        return Err(Status::new(400, "Invalid message id"));
    }
    match message::get_by_id(id, &conn) {
        Some(m) => {
            return Ok(Json(common_dtos::Message {
                id: m.id,
                room_id: m.room_id,
                sender_id: m.sender_id,
                content: m.content,
            }));
        }
        _ => {
            return Err(Status::new(500, "something went wrong"));
        }
    }
}

fn validate_create_message_request(request: &MessageCreateRequest) -> Option<&'static str> {
    if request.content.len() < 1 {
        Some("content cannot be empty")
    } else if request.room_id < 1 {
        Some("invalid room id")
    } else if request.sender_id < 1 {
        Some("invalid sender id")
    } else {
        None
    }
}
