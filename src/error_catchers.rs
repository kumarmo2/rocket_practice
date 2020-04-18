#[catch(400)]
pub fn bad_request() -> &'static str {
    "Bad Request"
}
