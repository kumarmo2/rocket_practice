use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub struct ApiKey(pub String);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
    BadCount,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // TODO: did not understand the code.
        let keys: Vec<_> = request.headers().get("Authorization").collect();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if keys[0] == "kumarmo2" => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
