use crate::business::user as user_bl;
use crate::dal::user;
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::responders::CorsResponder;
use crate::dtos::responders::CustomStatusResponse;
use crate::dtos::{CreateUserRequest, SignInRequest, UserDto};
use crate::models::MySqlDb;
use crate::utils::cookies::{delete_user_cookie, set_user_cookie};

use diesel::result::Error;

use chat_common_types::events::ClientEventQueueNameWrapper;

use manager::RabbitMqManager;

use rocket::{
    http::{Cookies, Status},
    State,
};

use std::ops::Deref;

use rocket_contrib::json::Json;
use validator::validate_email;

#[get("/<id>")]
pub fn get(_api_key: ApiKey, id: i32, conn: MySqlDb) -> Result<Json<UserDto>, Status> {
    if id < 1 {
        return Err(Status::new(400, "invalid user id"));
    }
    match user_bl::get_user_by_id(id, &conn) {
        Ok(user) => {
            println!("user: {:?}", user);
            Ok(Json(UserDto::from_user_model(user)))
        }
        Err(reason) => {
            println!("user not found: {}", reason);
            Err(Status::new(404, reason))
        }
    }
}

#[post("/signout")]
pub fn signout(mut cookies: Cookies) -> CustomStatusResponse {
    delete_user_cookie(&mut cookies);
    CustomStatusResponse::new(Status::Ok)
}

#[post("/signin", data = "<signin_request>")]
pub fn signin(
    signin_request: Json<SignInRequest>,
    conn: MySqlDb,
    mut cookies: Cookies,
) -> Result<Json<UserDto>, CustomStatusResponse> {
    if !validate_email(&signin_request.email) {
        return Err(CustomStatusResponse::new(Status::new(400, "Invalid email")));
    }
    if &signin_request.password == "" {
        return Err(CustomStatusResponse::new(Status::new(
            400,
            "Invalid password",
        )));
    }
    let user_model;
    match user_bl::get_user_by_email(&signin_request.email, &conn) {
        Ok(result) => {
            user_model = result;
        }
        Err(reason) => match reason {
            Error::NotFound => {
                return Err(CustomStatusResponse::new(Status::new(
                    404,
                    "email does not exists",
                )))
            }
            _ => return Err(CustomStatusResponse::new(Status::InternalServerError)),
        },
    }

    match bcrypt::verify(
        signin_request.deref().password.as_bytes(),
        &user_model.password,
    ) {
        Ok(is_match) => {
            if is_match {
                set_user_cookie(user_model.id, &mut cookies);
                return Ok(Json(UserDto::from_user_model(user_model)));
            }
            return Err(CustomStatusResponse::new(Status::new(
                401,
                "Email and password does not match",
            )));
        }
        Err(reason) => {
            println!("bcrypt::verify failed, reason: {}", reason);
            return Err(CustomStatusResponse::new(Status::new(
                500,
                "Something went wrong, try again",
            )));
        }
    }
}

#[post("/", data = "<user_request>")]
pub fn create(
    user_request: Json<CreateUserRequest>,
    mut cookies: Cookies,
    conn: MySqlDb,
) -> CustomStatusResponse {
    if !validate_email(&user_request.email) {
        return CustomStatusResponse::new(Status::new(400, "Invalid email"));
    }
    if let Some(age_input) = user_request.age {
        if age_input < 18 || age_input > 100 {
            return CustomStatusResponse::new(Status::new(400, "age must be between 18 and 100"));
        }
    }
    if user_request.password.len() < 6 || user_request.password.len() > 20 {
        return CustomStatusResponse::new(Status::new(
            400,
            "Password must be between 6 and 20 characters",
        ));
    }
    match user::get_by_email(&user_request.email, &conn) {
        Ok(_) => {
            println!("user found");
            return CustomStatusResponse::new(Status::new(409, "Email already exists"));
        }
        Err(reason) => match reason {
            Error::NotFound => {
                println!("no user found by the email: {}", &user_request.email);
            }
            _ => {
                return CustomStatusResponse::new(Status::new(
                    500,
                    "Something went wrong, try again",
                ));
            }
        },
    }
    let pass = String::from(&user_request.password);
    let hashed_pass = bcrypt::hash(pass, bcrypt::DEFAULT_COST).expect("could not created hash");
    match user::create_from_request(&user_request, &hashed_pass, &conn) {
        Ok(id) => {
            set_user_cookie(id, &mut cookies);
        }
        Err(reason) => {
            println!("could not create user: {}", reason);
        }
    }
    CustomStatusResponse::new(Status::Created)
}

// TODO: Remove this after testing because we are serving FE from the same site so this should not be needed.
#[options("/<id>/events/register")]
pub fn cors_for_register_events_endpoint(id: i32) -> CorsResponder {
    CorsResponder {}
}

#[post("/<id>/events/register")]
pub fn register_user_event_queue(
    _api_key: ApiKey,
    id: i32,
    conn: MySqlDb,
    rabbit: State<RabbitMqManager>,
) -> Result<Json<ClientEventQueueNameWrapper>, CustomStatusResponse> {
    if id < 1 {
        return Err(CustomStatusResponse::new(Status::new(
            400,
            "invalid user id",
        )));
    }
    match user_bl::get_user_by_id(id, &conn) {
        Ok(_) => {}
        Err(reason) => {
            return Err(CustomStatusResponse::new(Status::new(400, reason)));
        }
    }
    match user_bl::register_user(id, &rabbit, &conn) {
        Some(queue_name) => return Ok(Json(ClientEventQueueNameWrapper { queue_name })),
        None => {
            return Err(CustomStatusResponse::new(Status::new(
                500,
                "something went wrong, try again",
            )))
        }
    }
}
