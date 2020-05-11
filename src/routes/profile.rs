use crate::business::user::get_rooms;
use crate::dtos::{
    request_guards::UserAuthentication, responders::CustomStatusResponse, Profile, RoomDto,
};
use crate::models::{MySqlDb, Room, User};

use rocket::http::Status;

use rocket_contrib::json::Json;

#[get("/profile")]
pub fn get(
    user_authentication: UserAuthentication,
    conn: MySqlDb,
) -> Result<Json<Profile>, CustomStatusResponse> {
    let rooms: Vec<Room>;
    match get_rooms(user_authentication.id, &conn) {
        Ok(r) => {
            rooms = r;
        }
        Err(reason) => {
            println!("could not fetch rooms: {}", reason);
            return Err(CustomStatusResponse::new(Status::new(
                500,
                "something went wrong! Try Again",
            )));
        }
    }

    let rooms: Vec<RoomDto> = rooms
        .into_iter()
        .map(|model| RoomDto::from_room_model(model))
        .collect();
    let user: User = user_authentication.into();
    // TODO: should I make the fields as private?
    Ok(Json(Profile {
        user_id: user.id,
        name: user.name,
        email: user.email,
        rooms: Some(rooms),
    }))
}
