use crate::business::user as user_bl;
use crate::dal::user;
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::responders::CorsResponder;
use crate::dtos::CreateUserRequest;
use crate::models;
use crate::models::MySqlDb;
use chat_common_types::events::ClientEventQueueNameWrapper;
use manager::RabbitMqManager;
use rocket::http::{RawStr, Status};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/<name>/<age>/<cool>")]
pub fn big_hello(name: &RawStr, age: u32, cool: bool) -> String {
    if cool {
        format!("User: {}, of age: {} is cool", name, age)
    } else {
        format!("User: {}, of age: {} is not cool", name, age)
    }
}

#[get("/<name>")]
pub fn user_authorized_endpoint(apiKey: ApiKey, name: &RawStr) -> String {
    format!(
        "user with name: {} and apiKey: {}",
        name.to_string(),
        apiKey.0
    )
}

#[post("/", data = "<user_request>")]
pub fn create(apiKey: ApiKey, user_request: Json<CreateUserRequest>, conn: MySqlDb) -> Json<()> {
    match user::get_by_email(&user_request.email, &conn) {
        Some(user_model) => {
            println!("user found");
            return Json(());
        }
        None => {
            println!("no user found by the email: {}", &user_request.email);
        }
    }
    match user::create_from_request(&user_request, &conn) {
        Ok(_) => {
            println!("user created");
        }
        Err(reason) => {
            println!("could not create user: {}", reason);
        }
    }
    Json(())
}

#[options("/<id>/events/register")]
pub fn cors_for_register_events_endpoint(id: i32) -> CorsResponder {
    CorsResponder {}
}

#[post("/<id>/events/register")]
pub fn register_user_event_queue(
    api_key: ApiKey,
    id: i32,
    conn: MySqlDb,
    rabbit: State<RabbitMqManager>,
) -> Result<Json<ClientEventQueueNameWrapper>, Status> {
    if id < 1 {
        return Err(Status::new(400, "invalid user id"));
    }
    match user_bl::get_user_by_id(id, &conn) {
        Ok(_) => {}
        Err(reason) => {
            return Err(Status::new(400, reason));
        }
    }
    match user_bl::register_user(id, &rabbit, &conn) {
        Some(queue_name) => return Ok(Json(ClientEventQueueNameWrapper { queue_name })),
        None => return Err(Status::new(500, "something went wrong, try again")),
    }
}
