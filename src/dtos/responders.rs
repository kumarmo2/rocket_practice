use rocket::{
    http::Status,
    response::{Responder, Response, ResponseBuilder},
};

pub struct CorsResponder {}

impl<'r> Responder<'r> for CorsResponder {
    fn respond_to(self, _: &rocket::Request<'_>) -> Result<Response<'r>, Status> {
        let mut builder = ResponseBuilder::new(Response::new());
        builder.status(Status::NoContent);
        builder.raw_header("Access-Control-Allow-Origin", "*");
        builder.raw_header("Access-Control-Allow-Methods", "POST, OPTIONS");
        // TODO: restrict the headers.
        builder.raw_header("Access-Control-Allow-Headers", "*");
        let response = builder.finalize();
        Ok(response)
    }
}
