use rocket::{
    http::Status,
    response::{Responder, Response, ResponseBuilder},
};
use serde_json::to_string;
use std::collections::HashMap;
use std::io::Cursor;

pub struct CorsResponder {}

#[derive(Debug)]
pub struct CustomStatusResponse {
    status: Status,
}

impl CustomStatusResponse {
    pub fn new(status: Status) -> CustomStatusResponse {
        CustomStatusResponse { status }
    }
}

// TODO: Need to check if Rocket provides out-of-the-box solution for this.
impl<'r> Responder<'r> for CustomStatusResponse {
    fn respond_to(self, _: &rocket::Request<'_>) -> Result<Response<'r>, Status> {
        let mut map = HashMap::new();

        match self.status.code {
            200..=308 => {}
            400..=599 => {
                map.insert("error_message", self.status.reason);
            }
            _ => {}
        }

        let body = to_string(&map).unwrap();

        let reader = Cursor::new(body);
        let response = ResponseBuilder::new(Response::new())
            .status(self.status)
            .sized_body(reader)
            .finalize();
        return Ok(response);
    }
}

impl<'r> Responder<'r> for CorsResponder {
    fn respond_to(self, _: &rocket::Request<'_>) -> Result<Response<'r>, Status> {
        let mut builder = ResponseBuilder::new(Response::new());
        builder.status(Status::NoContent);
        builder.raw_header("Access-Control-Allow-Origin", "*");
        builder.raw_header("Access-Control-Allow-Methods", "POST");
        // TODO: restrict the headers.
        builder.raw_header("Access-Control-Allow-Headers", "*");
        Ok(builder.finalize())
    }
}
