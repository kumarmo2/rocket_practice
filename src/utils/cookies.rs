use crate::constants;
use crate::dtos::UserJwtPayload;
use jsonwebtoken::{Algorithm, Header};
use rocket::http::{Cookie, Cookies, SameSite};

pub fn set_user_cookie(user_id: i32, cookies: &mut Cookies) {
    let header = Header::new(Algorithm::HS256);
    let claims = UserJwtPayload {
        id: user_id,
        exp: 10000000000, // This was necessary for the library to decode.
    };
    let jwt = jsonwebtoken::encode(&header, &claims, constants::JWT_SECRET.as_bytes()).unwrap();
    set_cookie(constants::USER_AUTH_COOKIE_NAME.to_string(), jwt, cookies);
}

pub fn delete_user_cookie(cookies: &mut Cookies) {
    delete_cookie(constants::USER_AUTH_COOKIE_NAME, cookies);
}

fn delete_cookie(name: &'static str, cookies: &mut Cookies) {
    let mut cookie = Cookie::named(name);
    cookie.set_path("/");
    cookies.remove(cookie);
}

fn set_cookie(name: String, value: String, cookies: &mut Cookies) {
    let cookie = Cookie::build(name, value)
        // TODO: set the domain.
        // TODO: need to use secure cookie on production.
        .http_only(true) // cannot be accessed by javascript.
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(600)) // when maxAge and expires, both are set, maxAge takes precedence.
        .path("/")
        .finish();

    cookies.add(cookie);
}
