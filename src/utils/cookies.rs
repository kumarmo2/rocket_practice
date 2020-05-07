use crate::constants;
use jsonwebtoken::{Algorithm, Header};
use rocket::http::{Cookie, Cookies, SameSite};
use std::collections::HashMap;
use time::Duration;

pub fn set_user_cookie(user_id: i32, cookies: &mut Cookies) {
    let header = Header::new(Algorithm::HS256);
    let mut claims = HashMap::new();
    claims.insert("id", user_id);
    let jwt = jsonwebtoken::encode(&header, &claims, constants::JWT_SECRET.as_bytes()).unwrap();
    set_cookie(constants::USER_AUTH_COOKIE_NAME.to_string(), jwt, cookies);
}

fn set_cookie(name: String, value: String, cookies: &mut Cookies) {
    let cookie = Cookie::build(name, value)
        // TODO: need to use secure cookie on production.
        .http_only(true) // cannot be accessed by javascript.
        .same_site(SameSite::Lax)
        .expires(time::empty_tm() + Duration::days(30))
        .max_age(Duration::days(30))
        .path("/")
        .finish();

    cookies.add(cookie);
}
