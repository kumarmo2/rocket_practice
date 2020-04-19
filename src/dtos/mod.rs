pub mod request_guards;

use crate::models::Room;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     pub id: u32,
//     pub name: String,
//     pub age: u32,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    #[serde(rename = "creatorUserId")]
    pub creator_user_id: u32,
    #[serde(rename = "roomName")]
    pub room_name: Option<String>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub members: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: u32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
// pub struct RoomDto<'a> {
pub struct RoomDto {
    pub id: u32,
    pub name: String,
    pub path: String,
}

// impl<'a> RoomDto<'a> {
impl RoomDto {
    // pub fn from_room_model(model: &Room) -> RoomDto {
    pub fn from_room_model(model: Room) -> RoomDto {
        RoomDto {
            id: model.id,
            // name: &model.name,
            name: model.name,
            // path: &model.path,
            path: model.path,
        }
    }
}

// impl<'r> Responder<'r> for RoomDto<'_> {
impl<'r> Responder<'r> for RoomDto {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!(
                "Room's name: {}, id: {}, path: {}",
                self.name, self.id, self.path
            )))
            .ok()
    }
}
