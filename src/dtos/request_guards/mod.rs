pub mod ApiKey;
use crate::constants::{JWT_SECRET, USER_AUTH_COOKIE_NAME};
use crate::dtos::AuthoizationError;
use jsonwebtoken::{Algorithm, Validation};
use rocket::{http::Status, request::FromRequest, Outcome};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthentication {
    id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserAuthentication {
    type Error = Status; //TODO: Not sure for what this Error is used.
    fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::Outcome<Self, (Status, Self::Error), ()> {
        let cookies = request.cookies();
        let cookie;
        match cookies.get(USER_AUTH_COOKIE_NAME) {
            Some(c) => {
                cookie = c;
            }
            None => return Outcome::Failure((Status::Unauthorized, Status::Unauthorized)),
        }

        let result = jsonwebtoken::decode::<Self>(
            cookie.value(),
            JWT_SECRET.as_bytes(),
            &Validation::new(Algorithm::HS512),
        );
        // TODO: We should probably check in the DB(cache) for user availalibity.
        match result {
            Ok(token_data) => Outcome::Success(token_data.claims),
            Err(reason) => Outcome::Failure((Status::Unauthorized, Status::Unauthorized)),
        }
    }
}
