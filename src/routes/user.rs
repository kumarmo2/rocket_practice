use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::CreateUserRequest;
use crate::models::MySqlDb;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

// #[get("/user/<name>")]
// #[get("/<name>")]
// pub fn say_hello(name: &RawStr) -> String {
//     format!("Hello user, {}", name.as_str())
// }

// #[get("/user/<name>/<age>/<cool>")]
#[get("/<name>/<age>/<cool>")]
pub fn big_hello(name: &RawStr, age: u32, cool: bool) -> String {
    if cool {
        format!("User: {}, of age: {} is cool", name, age)
    } else {
        format!("User: {}, of age: {} is not cool", name, age)
    }
}

// #[get("/<id>/name")]
// pub fn get_user_by_id(id: Result<u32, &RawStr>) -> String {
//     match id {
//         Ok(u_id) => format!("user by id: {}", u_id),
//         Err(reason) => format!("error: {}", reason),
//     }
// }

#[get("/<name>")]
pub fn user_authorized_endpoint(apiKey: ApiKey, name: &RawStr) -> String {
    format!(
        "user with name: {} and apiKey: {}",
        name.to_string(),
        apiKey.0
    )
}

#[post("/", data = "<user_request>")]
pub fn create(
    apiKey: ApiKey,
    user_request: Json<CreateUserRequest>,
    conn: MySqlDb,
) -> Json<CreateUserRequest> {
    Json(user_request.0)
}
