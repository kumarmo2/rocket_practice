pub mod request_guards;
pub mod responders;

use crate::models::{self, Room};
use crate::schema::users;
use crate::schema::*;
use diesel::Insertable;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug)]
pub enum AuthoizationError {
    Missing,
    UnAuthorized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    #[serde(rename = "creatorUserId")]
    pub creator_user_id: i32,
    #[serde(rename = "roomName")]
    pub room_name: Option<String>,
    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct CreateUserRequest {
    pub name: String,
    pub age: Option<i32>,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomDto {
    pub id: i32,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMembersToRoomRequest {
    #[serde(rename = "memberIds")]
    member_ids: Vec<i32>,
}

#[derive(Insertable)]
#[table_name = "roomsubscribers"]
pub struct RoomSubscriberInsertableDto {
    pub member_id: i32,
    pub room_id: i32,
}

impl RoomSubscriberInsertableDto {
    pub fn new(member_id: i32, room_id: i32) -> Self {
        RoomSubscriberInsertableDto { member_id, room_id }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "messages"]
pub struct MessageCreateRequest {
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
}

impl MessageCreateRequest {
    pub fn new(room_id: i32, sender_id: i32, content: String) -> Self {
        Self {
            room_id,
            sender_id,
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub rooms: Option<Vec<RoomDto>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJwtPayload {
    pub id: i32,
    pub exp: u64,
}

impl UserDto {
    // I decided to took the ownership of User Model, because creation
    // of dto from the model should be the last step i.e After we have created the dto,
    // there should not be any more fiddling with the model. This is atleast what I believe
    // right now. If in future, I turn out to be wrong, will take a reference of the model instead.
    pub fn from_user_model(user: models::User) -> Self {
        UserDto {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

impl RoomDto {
    pub fn from_room_model(model: Room) -> RoomDto {
        RoomDto {
            id: model.id,
            name: model.name,
            path: model.url_identifier,
        }
    }
}

impl<'r> Responder<'r> for RoomDto {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!(
                "Room's name: {}, id: {}, path: {}",
                self.name, self.id, self.path
            )))
            .ok()
    }
}
