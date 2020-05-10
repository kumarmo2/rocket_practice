pub mod ApiKey;
use crate::business::user::get_user_by_id;
use crate::constants::{JWT_SECRET, USER_AUTH_COOKIE_NAME};
use crate::dtos::{AuthoizationError, UserJwtPayload};
use crate::models::MySqlDb;
use crate::models::User;
use jsonwebtoken::{Algorithm, Validation};
use rocket::{
    http::{Cookie, Status},
    request::FromRequest,
    Outcome,
};

use std::ops::Deref;

pub struct AdminAuthorization();

impl<'a, 'r> FromRequest<'a, 'r> for AdminAuthorization {
    type Error = AuthoizationError;
    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> Outcome<
        Self,
        (
            Status,
            <Self as rocket::request::FromRequest<'a, 'r>>::Error,
        ),
        (),
    > {
        match request.headers().get_one("Authorization") {
            Some(value) => {
                if value == "kumarmo2_admin" {
                    return Outcome::Success(AdminAuthorization());
                }
                return Outcome::Failure((Status::Unauthorized, AuthoizationError::UnAuthorized));
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, AuthoizationError::Missing));
            }
        }
    }
}

pub struct UserAuthentication(User);

impl Deref for UserAuthentication {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAuthentication {
    type Error = Status; //TODO: Not sure for what this Error is used.
    fn from_request(
        request: &rocket::Request<'r>,
    ) -> rocket::Outcome<Self, (Status, Self::Error), ()> {
        let cookies = request.cookies();
        let cookie: &Cookie;
        match cookies.get(USER_AUTH_COOKIE_NAME) {
            Some(c) => {
                cookie = c;
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
            }
        }

        // TODO: Refactoring needed.
        let result = jsonwebtoken::decode::<UserJwtPayload>(
            cookie.value(),
            JWT_SECRET.as_bytes(),
            &Validation::new(Algorithm::HS256),
        );
        let payload: UserJwtPayload;
        match result {
            Ok(token_data) => {
                payload = token_data.claims;
            }
            Err(_) => {
                return Outcome::Failure((Status::Unauthorized, Status::Unauthorized));
            }
        }
        let db = request.guard::<MySqlDb>().unwrap();

        match get_user_by_id(payload.id, &db) {
            Ok(user) => Outcome::Success(UserAuthentication(user)),
            Err(_) => Outcome::Failure((Status::Unauthorized, Status::Unauthorized)),
        }
    }
}
