pub mod ApiKey;
use crate::dtos::AuthoizationError;
use rocket::{http::Status, request::FromRequest, Outcome};

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
