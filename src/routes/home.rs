use crate::view_models::Home;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", Home {})
}

#[get("/dummy")]
pub fn dummy() -> &'static str {
    "kumarmo2"
}
