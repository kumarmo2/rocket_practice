#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/dummy")]
pub fn dummy() -> &'static str {
    "kumarmo2"
}
