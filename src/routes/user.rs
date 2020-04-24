use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::CreateUserRequest;
use crate::dal::{user};
use crate::models;
use crate::models::MySqlDb;
use rocket::http::RawStr;
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
