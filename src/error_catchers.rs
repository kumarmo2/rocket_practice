use crate::dtos::responders::CustomStatusResponse;
use rocket::http::Status;

#[catch(400)]
pub fn bad_request() -> &'static str {
    "Bad Request"
}

#[catch(401)]
pub fn unauthorized() -> CustomStatusResponse {
    CustomStatusResponse::new(Status::Unauthorized)
}
