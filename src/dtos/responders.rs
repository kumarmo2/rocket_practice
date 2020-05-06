use rocket::{
    http::Status,
    response::{Responder, Response, ResponseBuilder},
};
use std::io::Cursor;

pub struct CorsResponder {}

pub struct CustomStatusResponse {
    status: Status,
}

impl CustomStatusResponse {
    pub fn new(status: Status) -> CustomStatusResponse {
        CustomStatusResponse { status }
    }
}

impl<'r> Responder<'r> for CustomStatusResponse {
    fn respond_to(self, _: &rocket::Request<'_>) -> Result<Response<'r>, Status> {
        let reader = Cursor::new(self.status.reason);
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
