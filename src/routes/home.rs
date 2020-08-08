use crate::view_models::Empty;
use rocket_contrib::templates::Template;

// Right now, just a bare bone html is being served from the server.
// Rendering is still being done on client side. later on need to figure
// out how we can do SSR.
#[get("/")]
pub fn index() -> Template {
    // TODO: need to see if there is any alternate to providing the Empty struct.
    Template::render("index", Empty {})
}
// After we start doing server side rendering,
#[get("/rooms/<path>")]
pub fn chat_room(path: String) -> Template {
    // TODO: need to see if there is any alternate to providing the Empty struct.
    Template::render("index", Empty {})
}

#[get("/dummy")]
pub fn dummy() -> &'static str {
    "kumarmo2"
}
